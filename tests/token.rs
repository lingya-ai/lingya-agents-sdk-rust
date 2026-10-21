use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use futures_util::StreamExt;
use lingya_agents_sdk::models::{AiChatInput, ChatStreamProbeInput};
use lingya_agents_sdk::{AgentsClient, ApiError, Authentication, TokenProvider};

fn server(
    responses: Vec<(u16, &'static str, &'static str)>,
) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = thread::spawn(move || {
        let mut requests = Vec::new();
        for (status, content_type, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(10)))
                .unwrap();
            let mut bytes = Vec::new();
            loop {
                let mut buf = [0; 4096];
                let size = stream.read(&mut buf).unwrap();
                if size == 0 {
                    break;
                }
                bytes.extend_from_slice(&buf[..size]);
                if bytes.windows(4).any(|s| s == b"\r\n\r\n") {
                    break;
                }
            }
            requests.push(String::from_utf8(bytes).unwrap());
            write!(stream, "HTTP/1.1 {status} Response\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        }
        requests
    });
    (base, task)
}

fn auth(count: &Arc<AtomicUsize>) -> Authentication {
    let count = count.clone();
    Authentication::Token(TokenProvider::new(move |context| {
        assert_eq!(context.channel_id, "channel");
        let id = count.fetch_add(1, Ordering::SeqCst) + 1;
        async move { Ok(format!("fresh-{id}")) }
    }))
}

const SUBMISSION: &str = r#"{"conversationId":"conversation","messageId":"message","disposition":"queued","status":"pending"}"#;

#[tokio::test]
async fn concurrent_requests_get_independent_tokens_and_presign_never_issues() {
    let (base, task) = server(vec![(201, "application/json", SUBMISSION); 2]);
    let count = Arc::new(AtomicUsize::new(0));
    let client = AgentsClient::with_auth(base, "channel", auth(&count)).unwrap();
    let input = AiChatInput::new("hello".into());
    let chat = client.chat();
    let (first, second) = futures_util::join!(chat.create_chat(&input), chat.create_chat(&input));
    first.unwrap();
    second.unwrap();
    assert!(client
        .configuration()
        .create_open_api_token()
        .await
        .is_err());
    assert!(client.for_user("other").is_err());
    assert_eq!(count.load(Ordering::SeqCst), 2);
    let requests = task.join().unwrap();
    for (i, token) in ["fresh-1", "fresh-2"].iter().enumerate() {
        assert!(
            requests.iter().any(|r| r.contains(token)),
            "missing token {i}"
        );
    }
    for request in requests {
        let request = request.to_lowercase();
        assert!(request.contains("x-openapi-token:"));
        assert!(!request.contains("x-openapi-ak:"));
        assert!(!request.contains("x-openapi-user:"));
        assert!(!request.contains("x-openapi-signature:"));
    }
}

#[tokio::test]
async fn authentication_errors_do_not_retry_and_manual_retry_issues_again() {
    let (base, task) = server(vec![(401, "application/json", "{}"); 2]);
    let count = Arc::new(AtomicUsize::new(0));
    let client = AgentsClient::with_auth(base, "channel", auth(&count)).unwrap();
    for expected in 1..=2 {
        assert!(client
            .chat()
            .create_chat(&AiChatInput::new("hello".into()))
            .await
            .is_err());
        assert_eq!(count.load(Ordering::SeqCst), expected);
    }
    assert_eq!(task.join().unwrap().len(), 2);
}

#[tokio::test]
async fn hmac_options_bind_identity_and_keep_token_headers_absent() {
    let (base, task) = server(vec![(201, "application/json", SUBMISSION)]);
    let client = AgentsClient::with_auth(
        base,
        "channel",
        Authentication::Hmac {
            credentials: lingya_agents_sdk::OpenApiCredentials::new("access", "secret"),
            external_user_id: "user".into(),
        },
    )
    .unwrap();
    client
        .chat()
        .create_chat(&AiChatInput::new("hello".into()))
        .await
        .unwrap();
    let request = task.join().unwrap().remove(0).to_lowercase();
    assert!(request.contains("x-openapi-ak: access"));
    assert!(request.contains("x-openapi-user: dxnlcg"));
    assert!(!request.contains("x-openapi-token:"));
}

#[tokio::test]
async fn invalid_tokens_and_callback_errors_are_redacted_without_sending() {
    for token in ["", "secret\r\nheader", "invalid token", "中文"] {
        let client = AgentsClient::with_auth(
            "http://127.0.0.1:1",
            "channel",
            Authentication::Token(TokenProvider::new(
                move |_| async move { Ok(token.to_owned()) },
            )),
        )
        .unwrap();
        let error = client
            .chat()
            .create_chat(&AiChatInput::new("hello".into()))
            .await
            .unwrap_err();
        assert!(matches!(error, ApiError::InvalidInput(_)));
        assert!(!format!("{error:?}").contains("secret"));
    }
    let client = AgentsClient::with_auth(
        "http://127.0.0.1:1",
        "channel",
        Authentication::Token(TokenProvider::new(|_| async {
            Err(ApiError::InvalidInput("secret-token".into()))
        })),
    )
    .unwrap();
    let error = client
        .chat()
        .create_chat(&AiChatInput::new("hello".into()))
        .await
        .unwrap_err();
    assert!(!format!("{error:?}").contains("secret-token"));
    assert!(!format!("{client:?}").contains("secret-token"));
}

#[tokio::test]
async fn sse_issues_per_connection_not_per_event() {
    let body = "data: {\"probeId\":\"probe\",\"sequence\":1,\"serverElapsedMs\":0}\n\ndata: {\"probeId\":\"probe\",\"sequence\":2,\"serverElapsedMs\":1}\n\n";
    let (base, task) = server(vec![(200, "text/event-stream", body); 2]);
    let count = Arc::new(AtomicUsize::new(0));
    let client = AgentsClient::with_auth(base, "channel", auth(&count)).unwrap();
    for expected in 1..=2 {
        let input = ChatStreamProbeInput::new("probe".into());
        let mut stream = client
            .chat()
            .probe_event_stream(&input, &Default::default())
            .await
            .unwrap();
        let mut events = 0;
        while let Some(event) = stream.next().await {
            event.unwrap();
            events += 1;
        }
        assert_eq!(events, 2);
        assert_eq!(count.load(Ordering::SeqCst), expected);
    }
    task.join().unwrap();
}

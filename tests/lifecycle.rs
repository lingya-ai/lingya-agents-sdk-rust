use lingya_agents_sdk::{AgentsClient, Authentication, RequestOptions, TokenProvider};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

#[tokio::test]
async fn timeout_and_cancellation_drop_pending_token_without_sending() {
    let entered = Arc::new(AtomicUsize::new(0));
    let count = entered.clone();
    let client = AgentsClient::with_auth(
        "http://127.0.0.1:1",
        "channel",
        Authentication::Token(TokenProvider::new(move |_| {
            count.fetch_add(1, Ordering::SeqCst);
            async { std::future::pending().await }
        })),
    )
    .unwrap();
    let options = RequestOptions {
        timeout: Some(Duration::from_millis(20)),
        ..Default::default()
    };
    let error = client
        .with_request_options(options)
        .configuration()
        .get_agents_config()
        .await
        .unwrap_err();
    assert_eq!(error.kind(), "timeout");
    let options = RequestOptions::default();
    options.cancellation.cancel();
    let error = client
        .with_request_options(options)
        .configuration()
        .get_agents_config()
        .await
        .unwrap_err();
    assert_eq!(error.kind(), "cancelled");
    assert_eq!(entered.load(Ordering::SeqCst), 1);
}

#[test]
fn sse_handles_split_utf8_cr_and_incomplete_eof() {
    let mut decoder = lingya_agents_sdk::sse::SseDecoder::default();
    let input = "data: 中\r\rdata: incomplete".as_bytes();
    let mut values = Vec::new();
    for byte in input {
        values.extend(decoder.push(&[*byte]).unwrap());
    }
    assert_eq!(values, ["中"]);
    assert!(decoder.finish().unwrap().is_empty());
}

#[tokio::test]
async fn websocket_completes_without_end_and_each_connection_uses_a_new_token() {
    use futures_util::StreamExt;
    use lingya_agents_sdk::{models::AiChatStreamInput, StreamTransport};
    use tokio_tungstenite::tungstenite::{
        accept_hdr,
        handshake::server::{Request, Response},
        Message,
    };
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let mut tokens = Vec::new();
        for _ in 0..2 {
            let (stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            // tungstenite requires this concrete HTTP error response in its callback.
            #[allow(clippy::result_large_err)]
            let mut socket = accept_hdr(stream, |request: &Request, mut response: Response| {
                let token = request
                    .headers()
                    .get("Sec-WebSocket-Protocol")
                    .unwrap()
                    .clone();
                tokens.push(token.to_str().unwrap().to_owned());
                response
                    .headers_mut()
                    .insert("Sec-WebSocket-Protocol", token);
                Ok(response)
            })
            .unwrap();
            for frame in [
                r#"{"id":"m","event":"ready"}"#,
                r#"{"id":"m","event":"future","data":{"type":"future","kept":true}}"#,
                r#"{"id":"m","event":"complete"}"#,
            ] {
                socket.send(Message::Text(frame.into())).unwrap();
            }
            let _ = socket.read();
            let _ = socket.flush();
        }
        tokens
    });
    let issued = Arc::new(AtomicUsize::new(0));
    let count = issued.clone();
    let client = AgentsClient::with_auth(
        base,
        "c",
        Authentication::Token(TokenProvider::new(move |_| {
            let token = format!("fresh-{}", count.fetch_add(1, Ordering::SeqCst) + 1);
            async move { Ok(token) }
        })),
    )
    .unwrap()
    .with_request_options(RequestOptions {
        timeout: Some(Duration::from_secs(2)),
        transport: StreamTransport::WebSocket,
        ..Default::default()
    });
    for _ in 0..2 {
        let mut subscription = client
            .chat()
            .subscribe_chat_events("c", &AiChatStreamInput::new("m".into()))
            .await
            .unwrap();
        let event = subscription.next().await.unwrap().unwrap();
        assert_eq!(event.event_type(), "future");
        assert!(subscription.next().await.is_none());
        assert_eq!(subscription.closed().await.reason, "complete");
    }
    assert_eq!(issued.load(Ordering::SeqCst), 2);
    assert_eq!(
        server.join().unwrap(),
        ["xlt-openapi-token.fresh-1", "xlt-openapi-token.fresh-2"]
    );
}

#[test]
fn shared_http_error_vectors_keep_business_fields_and_request_identity() {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vectors {
        http_errors: Vec<Vector>,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vector {
        status: u16,
        body: Box<serde_json::value::RawValue>,
        request_id: String,
    }
    let vectors: Vectors =
        serde_json::from_str(include_str!("../test-vectors/lifecycle-v1.json")).unwrap();
    for vector in vectors.http_errors {
        let error = lingya_agents_sdk::ApiError::Http {
            method: "GET".into(),
            path: "/config".into(),
            status: reqwest::StatusCode::from_u16(vector.status).unwrap(),
            response_body: vector.body.get().into(),
            request_id: Some(vector.request_id.clone()),
        };
        assert_eq!(error.kind(), "http");
        assert_eq!(error.status(), Some(vector.status));
        assert_eq!(error.message(), "fixture failure");
        assert_eq!(error.fields()[0].field, "title");
        assert_eq!(error.request_id(), Some(vector.request_id.as_str()));
    }
}

#[tokio::test]
async fn explicit_close_and_parent_cancellation_release_owned_websockets() {
    use futures_util::StreamExt;
    use lingya_agents_sdk::{models::AiChatStreamInput, OpenApiCredentials, StreamTransport};
    use tokio_tungstenite::tungstenite::{accept, Message};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        for _ in 0..2 {
            let (stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut socket = accept(stream).unwrap();
            socket
                .send(Message::Text(r#"{"id":"m","event":"ready"}"#.into()))
                .unwrap();
            let _ = socket.read();
        }
    });
    let client = AgentsClient::with_auth(
        base,
        "c",
        Authentication::Hmac {
            credentials: OpenApiCredentials::new("fixture-ak", "fixture-sk"),
            external_user_id: "u".into(),
        },
    )
    .unwrap();
    let options = RequestOptions {
        transport: StreamTransport::WebSocket,
        ..Default::default()
    };
    let cancellation = options.cancellation.clone();
    let mut subscription = client
        .with_request_options(options)
        .chat()
        .subscribe_chat_events("c", &AiChatStreamInput::new("m".into()))
        .await
        .unwrap();
    cancellation.cancel();
    assert_eq!(
        subscription.next().await.unwrap().unwrap_err().kind(),
        "cancelled"
    );
    assert_eq!(subscription.closed().await.reason, "cancelled");
    let options = RequestOptions {
        transport: StreamTransport::WebSocket,
        ..Default::default()
    };
    let parent = options.cancellation.clone();
    let mut subscription = client
        .with_request_options(options)
        .chat()
        .subscribe_chat_events("c", &AiChatStreamInput::new("m".into()))
        .await
        .unwrap();
    subscription.close();
    assert!(subscription.next().await.is_none());
    assert_eq!(subscription.closed().await.reason, "closed");
    assert!(!parent.is_cancelled());
    server.join().unwrap();
}

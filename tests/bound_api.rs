use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use lingya_agents_sdk::models::AiChatInput;
use lingya_agents_sdk::{AgentsClient, OpenApiCredentials};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Operation {
    #[serde(rename = "operationId")]
    operation_id: String,
}

#[test]
fn every_contract_operation_has_one_bound_method_without_channel_id() {
    let manifest: Vec<Operation> = serde_json::from_str(
        &fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/openapi/endpoints.json"
        ))
        .unwrap(),
    )
    .unwrap();
    let source =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bound_api.rs")).unwrap();
    let declaration = Regex::new(r"pub async fn ([a-z0-9_]+)\s*\((?s:.*?)\) -> Result").unwrap();
    let methods: Vec<_> = declaration
        .captures_iter(&source)
        .map(|capture| capture[1].to_owned())
        .collect();

    assert_eq!(manifest.len(), 46);
    assert_eq!(methods.len(), 46);
    for operation in manifest {
        assert!(methods.contains(&camel_to_snake(&operation.operation_id)));
    }
    for signature in declaration.captures_iter(&source) {
        assert!(!signature[0].contains("channel_id"));
    }
}

#[tokio::test]
async fn root_channel_is_encoded_and_injected_once() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 8192];
        let size = stream.read(&mut request).unwrap();
        let request = String::from_utf8_lossy(&request[..size]).into_owned();
        let body = r#"{"conversationId":"conversation-1","messageId":"message-1","disposition":"queued","status":"pending"}"#;
        write!(
            stream,
            "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
        request
    });
    let user = AgentsClient::new(
        format!("http://{address}"),
        "channel/一",
        OpenApiCredentials::new("abcdefghijklmnopqrstuvwxyzABCDEF", "test-secret"),
    )
    .unwrap()
    .for_user("external-user")
    .unwrap();

    let result = user
        .chat()
        .create_chat(&AiChatInput::new("你好".to_owned()))
        .await
        .unwrap();
    let request = server.join().unwrap();

    assert_eq!(result.message_id, "message-1");
    assert!(request
        .starts_with("POST /api/agents/channel/openapi/v1/channel%2F%E4%B8%80/chat HTTP/1.1"));
}

fn camel_to_snake(value: &str) -> String {
    let mut result = String::new();
    for character in value.chars() {
        if character.is_ascii_uppercase() {
            result.push('_');
            result.push(character.to_ascii_lowercase());
        } else {
            result.push(character);
        }
    }
    result
}

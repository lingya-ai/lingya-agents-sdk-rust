# 灵涯 Agents Rust SDK
Lingya Agents SDK for Rust

用于在 Rust 可信服务端调用灵涯 Agents OpenAPI。
Use this SDK to call Lingya Agents OpenAPI from a trusted Rust server.

## 安装
Installation

```toml
[dependencies]
lingya-agents-sdk = "0.4.0"
futures-util = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## 快速开始
Quick start

```rust,no_run
use lingya_agents_sdk::models::AiChatInput;
use lingya_agents_sdk::{AgentsClient, OpenApiCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AgentsClient::new(
        "https://lingtong.lingya.tech/",
        std::env::var("OPENAPI_CHANNEL_ID")?,
        OpenApiCredentials::new(
            std::env::var("OPENAPI_AK")?,
            std::env::var("OPENAPI_SK")?,
        ),
    )?;
    let user = client.for_user("external-user-id")?;
    let submission = user
        .chat()
        .create_chat(&AiChatInput::new("你好".into()))
        .await?;
    println!("{}", submission.conversation_id);
    Ok(())
}
```

`channel_id` 只在创建 `AgentsClient` 时提供，业务方法不再接收它。
Provide `channel_id` only when creating `AgentsClient`; business methods do not accept it.

## SSE 事件流
SSE event stream

```rust,no_run
use futures_util::StreamExt;
use lingya_agents_sdk::models::AiChatStreamInput;
use lingya_agents_sdk::StreamChatEventsOptions;

let mut events = user
    .chat()
    .stream_chat_events(
        &submission.conversation_id,
        &AiChatStreamInput::new(submission.message_id.clone()),
        &StreamChatEventsOptions::default(),
    )
    .await?;

while let Some(event) = events.next().await {
    println!("{:?}", event?);
}
```

## API 分组
API groups

可用分组为 `chat()`、`configuration()`、`conversations()`、`events()`、`files()`、`interactions()`、`knowledge()`、`messages()`、`sql()` 和 `workspace()`。
Available groups are `chat()`, `configuration()`, `conversations()`, `events()`, `files()`, `interactions()`, `knowledge()`, `messages()`, `sql()`, and `workspace()`.

## 错误处理
Error handling

```rust,no_run
use lingya_agents_sdk::ApiError;

match user.conversations().get_conversation_title("conversation-id").await {
    Ok(title) => println!("{}", title.title),
    Err(ApiError::Http { status, response_body, .. }) => eprintln!("HTTP {status}: {response_body}"),
    Err(error) => return Err(error.into()),
}
```

请只在可信服务端保存 `secret_key`，不要将其放入浏览器、移动端、桌面端或日志。
Keep `secret_key` on trusted servers only; never put it in browsers, mobile apps, desktop apps, or logs.

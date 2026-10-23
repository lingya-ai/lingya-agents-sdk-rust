# 灵涯 Agents Rust SDK
Lingya Agents SDK for Rust

用于在 Rust 可信服务端调用灵涯 Agents OpenAPI。
Use this SDK to call Lingya Agents OpenAPI from a trusted Rust server.

## 安装
Installation

```toml
[dependencies]
lingya-agents-sdk = "1.0.0"
futures-util = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## 一次性令牌回调

`with_auth` 在构造时绑定认证，支持异步 token 回调和 HMAC。原有 `new(...credentials).for_user(...)` 保持兼容。
Use with_auth for bound authentication and asynchronous token callbacks. Legacy constructors remain available.

```rust
use lingya_agents_sdk::{AgentsClient, Authentication, TokenProvider};

let client = AgentsClient::with_auth(
    base_url,
    channel_id,
    Authentication::Token(TokenProvider::new(|context| async move {
        // 自己的业务接口：每次签发新的 token，错误返回 ApiError。
        issue_fresh_token(context.channel_id).await
    })),
)?;
let config = client.configuration().get_agents_config().await?;
```

HMAC 模式使用 `Authentication::Hmac { credentials, external_user_id }`。`with_auth` 已绑定身份，不能再调用 `for_user`；旧 `new` 构造则必须先调用 `for_user`，才能使用返回的用户客户端发起请求。
Use Authentication::Hmac for trusted-server credentials and a user ID. Identity-bound clients cannot switch users. Legacy new clients require for_user before making calls.

令牌有效期十分钟，但只允许认证一次；每次请求、手动重试和 SSE/WebSocket 重新连接都独立调用回调，不缓存、不合并并发签发、不自动重试或跟随重定向。认证成功后即消费，业务失败不能复用。Token 模式不携带 HMAC 头，不能调用 `createOpenApiToken`；需由持有 AK/SK 的可信服务端签发。回调异常及非法 token 会在业务请求发出前拒绝。`/api/agents-token`、`issue_fresh_token` / `issueFreshToken` 都是业务系统示意，并非 SDK 新增接口。签发服务应从登录态确定用户和允许的渠道。
Tokens expire after ten minutes but authenticate only once. Each request, explicit retry, and SSE/WebSocket reconnection invokes the provider independently. Token clients do not cache, share issuance, retry automatically, follow redirects, send HMAC headers, or call presign. Use a trusted server to issue tokens for the authenticated business user and permitted channel; the example issuer functions/routes belong to your application.

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
match user.conversations().get_conversation_title("conversation-id").await {
    Ok(title) => println!("{}", title.title),
    Err(error) => {
        eprintln!("{:?}: {:?}, {:?}, {:?}", error.kind(), error.status(), error.code(), error.request_id());
        return Err(error.into());
    },
}
```

请只在可信服务端保存 `secret_key`，不要将其放入浏览器、移动端、桌面端或日志。
Keep `secret_key` on trusted servers only; never put it in browsers, mobile apps, desktop apps, or logs.


## 0.7 请求生命周期与订阅

```rust
use lingya_agents_sdk::{RequestOptions, StreamTransport};
use std::time::Duration;
use futures_util::StreamExt;

let scoped = client.with_request_options(RequestOptions {
    timeout: Some(Duration::from_secs(60)),
    idle_timeout: Some(Duration::from_secs(30)),
    transport: StreamTransport::WebSocket,
    ..Default::default()
});
let mut subscription = scoped.chat().subscribe_chat_events(&conversation_id, &input).await?;
while let Some(event) = subscription.next().await { consume(event?); }
let result = subscription.closed().await;
```
订阅默认使用 SSE；选择 WebSocket 时设置 `RequestOptions { transport: StreamTransport::WebSocket, ..Default::default() }`。令牌回调上下文携带 `CancellationToken`；可通过 `with_http_client` 注入共享 reqwest 客户端。

超时覆盖获取令牌、建连和完整响应；SDK 不自动重试或重连。业务 End 立即释放连接，WebSocket complete/1000 不补造 End。错误包含可判别分类、HTTP 状态和可取得的业务错误及请求标识。详见 [生命周期、错误和生成机制](docs/request-lifecycle.md)。运行时由契约项目统一生成，修改生成产物会被 CI 检出。

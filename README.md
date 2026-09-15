# 灵涯 Agents Rust SDK
Lingya Agents SDK for Rust

这是灵涯 Agents OpenAPI 的异步服务端 Rust SDK；模型来自固定契约提交，覆盖全部 46 个公开路由，并内置 `OPENAPI-HMAC-SHA256-V1` 签名、SSE 分片解码和未知判别值后备类型。
This asynchronous server-side Rust SDK for Lingya Agents OpenAPI pins its contract revision, covers all 46 public routes, and includes `OPENAPI-HMAC-SHA256-V1` signing, chunked SSE decoding, and explicit fallbacks for unknown discriminator values.

仅适用于可信服务端；不要把 secret 放入浏览器、桌面端、移动端、日志或异常。
Use this SDK only on trusted servers. Never put the secret in browsers, desktop or mobile applications, logs, or errors.

## 安装
Installation

```toml
[dependencies]
lingya-agents-sdk = "0.1.2"
futures-util = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

国内网络可使用项目自带的 rsproxy sparse 配置。
For networks in mainland China, use the repository's built-in rsproxy sparse configuration.

## 调用与 SSE
Calls and SSE

```rust,no_run
use futures_util::StreamExt;
use lingya_agents_sdk::models::{AiChatInput, AiChatSubmission};
use lingya_agents_sdk::{HttpMethod, LingyaAgentsClient, OpenApiCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LingyaAgentsClient::new(
        "https://lingtong.lingya.tech",
        "channel-id",
        OpenApiCredentials::new("access-key", "secret-key"),
    )?;
    let user = client.for_user("your-system-user-id")?;
    let submission: AiChatSubmission = user
        .send_model(HttpMethod::POST, "", &AiChatInput::new("你好".into()), &[])
        .await?;
    let mut events = user
        .stream_chat_events(&submission.conversation_id, &submission.message_id)
        .await?;
    while let Some(event) = events.next().await {
        println!("{}", event?.event_type());
    }
    Ok(())
}
```

`QueryParameter` 保留参数顺序并支持重复名称；`get_model` 与 `send_model` 只返回调用方指定的明确 `Deserialize` 类型，二进制下载使用 `request_bytes`。
`QueryParameter` preserves parameter order and supports duplicate names. `get_model` and `send_model` return only the explicit `Deserialize` types selected by the caller, while binary downloads use `request_bytes`.

15 种已知事件和 12 种已知工具扩展均映射为明确枚举；服务端新增判别值时，以 `raw_json: String` 保留原始对象。
All 15 known events and 12 known tool extensions map to explicit enums. When the server adds a discriminator value, the original object is retained in `raw_json: String`.

## 开发验证
Development verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo package --locked
```

真实测试还需设置 `OPENAPI_AK`、`OPENAPI_SK`、`LINGYA_LIVE_BASE_URL` 与 `LINGYA_LIVE_CHANNEL_ID`，然后运行 `cargo test --test live -- --nocapture`。
Live tests also require `OPENAPI_AK`, `OPENAPI_SK`, `LINGYA_LIVE_BASE_URL`, and `LINGYA_LIVE_CHANNEL_ID`; run them with `cargo test --test live -- --nocapture`.

测试会逐一核对契约中的 46 个 method/path，并在 `build/reports/live-api/` 生成脱敏报告。
The tests verify every one of the 46 contract method/path pairs and write a redacted report to `build/reports/live-api/`.

契约来源为 `lingya-ai/lingya-agents-openapi@7362df0`，OpenAPI Generator 版本为 `7.25.0`。
The contract source is `lingya-ai/lingya-agents-openapi@7362df0`, and the OpenAPI Generator version is `7.25.0`.

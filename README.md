# Lingya Agents SDK for Rust

灵芽 Agents OpenAPI 的异步服务端 Rust SDK。模型来自固定契约提交，覆盖全部 46 个公开路由，并内置 `OPENAPI-HMAC-SHA256-V1` 签名、SSE 分片解码和未知判别值 fallback。

> 仅适用于可信服务端。不要把 secret 放入浏览器、桌面端、移动端、日志或异常。

## 安装

```toml
[dependencies]
lingya-agents-sdk = "0.1.2"
futures-util = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

国内网络可使用项目自带的 rsproxy sparse 配置。

## 调用

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

`QueryParameter` 保留参数顺序并支持重复名称。`get_model` 与 `send_model` 只返回调用方指定的明确 `Deserialize` 类型；二进制下载使用 `request_bytes`。15 种已知事件和 12 种已知工具扩展均映射为明确枚举，服务端新增判别值则以 `raw_json: String` 保留原始对象。

## 开发验证

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo package --locked
```

真实测试还需要设置 `OPENAPI_AK`、`OPENAPI_SK`、`LINGYA_LIVE_BASE_URL` 与 `LINGYA_LIVE_CHANNEL_ID`，然后运行 `cargo test --test live -- --nocapture`。测试逐一核对契约中的 46 个 method/path，并在 `build/reports/live-api/` 生成脱敏报告。

契约来源：`lingya-ai/lingya-agents-openapi@7362df0`，OpenAPI Generator `7.25.0`。

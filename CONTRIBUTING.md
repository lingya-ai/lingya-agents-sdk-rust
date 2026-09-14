# Contributing

请先提交 Issue 说明变更动机。提交前运行 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --all-targets` 和 `cargo package --locked`。

模型由固定版本的 OpenAPI 契约生成；不要直接修改生成模型。真实服务测试必须通过环境变量提供凭证。

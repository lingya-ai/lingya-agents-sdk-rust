# Contributing

请先提交 Issue 说明变更动机。提交前运行 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --all-targets` 和 `cargo package --locked`。

模型由固定版本的 OpenAPI 契约生成；不要直接修改生成模型。真实服务测试必须通过环境变量提供凭证。

## Canonical SDK generation

Edit runtime sources, facade generators, templates, and generation inputs in `lingya-agents-openapi/sdk/rust/`; edit the public contract in `lingya-agents-openapi/openapi/lingya-agents-v1.yaml`. Run `npm ci && npm run generate:sdks` from that repository (`sync:sdks` is an alias), then run this SDK's checks. Commit the generated output and `SDK_GENERATION.json` together. Do not edit managed runtime files directly or use standalone generators as the complete regeneration workflow. Ordinary builds use committed sources.

See [request lifecycle and generation](docs/request-lifecycle.md) for cancellation, fresh-token callbacks, streaming, and generation checks. Live-service tests require environment credentials.

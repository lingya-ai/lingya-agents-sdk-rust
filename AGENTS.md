# Rust SDK 发布指南

## 契约同步

契约、运行时和生成输入的唯一源是 `lingya-agents-openapi`。先在契约仓库修改 `openapi/`、`sdk/rust/` 中对应的源文件、模板或配置，再运行 `npm ci && npm run generate:sdks`；`npm run sync:sdks` 是兼容入口。该流程统一生成契约快照、传输清单、模型/API、运行时和业务门面，并更新 `SDK_GENERATION.json`，无需在 SDK 仓库单独执行旧生成命令。

生成管理的文件不要直接修改。普通构建使用已提交源码，不触发重新生成。生成检查使用固定契约 tag，验证全部受管理文件及连续生成的一致性；升级时同步生成工作流中的契约引用。详见 `docs/request-lifecycle.md`。

## 版本与检查

在契约仓库 `sdk/rust/` 更新 `Cargo.toml`、`Cargo.lock`、`openapi-generator-config.json` 的 `packageVersion`，统一重新生成；同步 SDK README 依赖示例和 CHANGELOG。依次运行 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --all-targets` 和 `cargo package --locked`。

## 发布

提交并推送到 `main`，确认 CI 通过后创建并推送 `vX.Y.Z` tag。GitHub Actions 会重新运行完整检查，使用 `CARGO_REGISTRY_TOKEN` 发布到 crates.io 并创建 GitHub Release；按需批准 `release` 环境。最后确认 crates.io 上的版本号。

# Rust SDK 发布指南

## 契约同步

先确认 `lingya-agents-openapi` 已发布所需的 `vX.Y.Z`，再同步 `openapi/lingya-agents-v1.yaml`、`openapi/endpoints.json` 和 `CONTRACT_VERSION`。将 CI 与 release workflow 的契约引用更新到同一 tag。运行 `scripts/generate_bound_api.py`；新增或修改生成模型时，确认 `src/models/mod.rs` 导出了所有新模型。

## 版本与检查

同步更新 `Cargo.toml`、`Cargo.lock`、`openapi-generator-config.json` 的 `packageVersion`、README 依赖示例和 CHANGELOG。依次运行 `cargo fmt --all -- --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --all-targets` 和 `cargo package --locked`。

## 发布

提交并推送到 `main`，确认 CI 通过后创建并推送 `vX.Y.Z` tag。GitHub Actions 会重新运行完整检查，使用 `CARGO_REGISTRY_TOKEN` 发布到 crates.io 并创建 GitHub Release；按需批准 `release` 环境。最后确认 crates.io 上的版本号。

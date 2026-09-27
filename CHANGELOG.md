# Changelog

## 0.5.0 - 2026-09-27

- 同步 OpenAPI 契约 v0.1.5，新增异步任务同步及子 Agent 查询、结果和取消接口。
- 更新公开 API 封装与生成模型，使其匹配当前契约结构。

## 0.4.0

- Export `AgentsClient`, `AgentsUserClient`, `ApiError`, unprefixed event types, and unprefixed group facades.
- Keep the 0.3.x `Lingya*` names as deprecated compatibility exports.
- Rewrite the README as a usage-only bilingual guide.

## 0.3.0

- Bind `channel_id` once and expose all 46 operations through generated, grouped facades.
- Add explicit query/header option structs and keep generic HTTP helpers deprecated until 1.0.
- Generate public operation signatures from contract 0.1.3 metadata.

## 0.1.2

- 首个 Rust SDK：覆盖 46 个 Agents OpenAPI 路由、HMAC-SHA256-V1、异步请求、SSE 和强类型模型。
- 15 种事件与 12 种工具扩展使用明确枚举，未知判别值保留原始 JSON 字符串。

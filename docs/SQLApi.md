# SQLApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `getSqlQueryResult` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/sql-query-results/{resultId}` | 分页读取 SQL 结果 / Get paged SQL results | — | SqlQueryResultPage |
| `getSqlQueryChartData` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/sql-query-results/{resultId}/chart-data` | 读取 SQL 图表数据 / Get SQL chart data | — | SqlChartDataset |
| `exportSqlQueryResult` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/sql-query-results/{resultId}/export` | 导出 SQL 结果 / Export SQL results | — | text/csv |


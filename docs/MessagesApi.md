# MessagesApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `listConversationMessages` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages` | 分页查询会话消息 / List conversation messages | — | ConversationMessagePage |
| `getConversationMessage` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages/{messageId}` | 读取单条会话消息 / Get a conversation message | — | ConversationMessage |
| `listConversationAsyncTasks` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/async-tasks` | 分页查询异步任务 / List asynchronous tasks | — | AsyncTaskPage |
| `syncConversationAsyncTasks` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/async-tasks/sync` | 增量同步异步任务 / Sync asynchronous task updates | — | AgentAsyncTaskSync |
| `getConversationAsyncTask` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/async-tasks/{asyncTaskId}` | 读取异步任务 / Get an asynchronous task | — | AsyncTask |
| `listConversationSubagents` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/subagents` | 分页查询子 Agent 任务 / List subagent tasks | — | SubagentTaskPage |
| `syncConversationSubagents` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/subagents/sync` | 增量同步子 Agent 状态 / Sync subagent task updates | — | SubagentTaskSync |
| `getConversationSubagent` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/subagents/{subagentTaskId}` | 读取子 Agent 状态 / Get a subagent task | — | SubagentTask |
| `cancelConversationSubagent` | `DELETE /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/subagents/{subagentTaskId}` | 取消子 Agent 任务 / Cancel a subagent task | — | SubagentTask |
| `getConversationSubagentResult` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/subagents/{subagentTaskId}/result` | 读取子 Agent 结果 / Get a subagent result | — | SubagentTaskResult |
| `cancelQueuedMessage` | `DELETE /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages/{messageId}/queue` | 取消排队消息 / Cancel a queued message | — | ConversationMessage |
| `syncConversationMessages` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages/sync` | 增量同步消息 / Sync conversation messages | — | ConversationMessageSync |


# ChatApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `createChat` | `POST /api/agents/channel/openapi/v1/{channelId}/chat` | 创建会话并提交消息 / Create a conversation and submit a message | AiChatInput | AiChatSubmission |
| `continueChat` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}` | 向已有会话提交消息 / Submit a message to an existing conversation | AiChatInput | AiChatSubmission |
| `streamChatEvents` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/stream` | 订阅消息事件流 / Stream message events | AiChatStreamInput | AiChatBriefEvent |
| `probeEventStream` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/stream-probe` | 探测 SSE 连接 / Probe the SSE connection | ChatStreamProbeInput | ChatStreamProbeEvent |
| `interruptConversation` | `DELETE /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/interrupt` | 中断会话执行 / Interrupt conversation execution | — | — |
| `compactConversation` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/compact` | 压缩会话上下文 / Compact conversation context | — | — |
| `getGoal` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal` | 读取当前目标 / Get current goal | — | AgentGoalView |
| `editGoal` | `PATCH /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal` | 编辑目标 / Edit goal | EditAgentGoalInput | AgentGoal |
| `getGoalHistory` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal/history` | 查询目标历史 / Get goal history | — | AgentGoalHistory |
| `pauseGoal` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal/pause` | 暂停目标 / Pause goal | AgentGoalRefInput | AgentGoal |
| `resumeGoal` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal/resume` | 恢复目标 / Resume goal | AgentGoalRefInput | AgentGoal |
| `clearGoal` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/goal/clear` | 清除目标 / Clear goal | AgentGoalRefInput | AgentGoal |

高级订阅 subscribeChatEvents / subscribe_chat_events 默认 SSE，可选择 WebSocket；原 stream 方法兼容保留。SSE 收到 End 后立即关闭，WebSocket complete/1000 正常结束且不补造 End。

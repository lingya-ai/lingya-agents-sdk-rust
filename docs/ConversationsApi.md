# ConversationsApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `deleteConversation` | `DELETE /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}` | 删除会话 / Delete a conversation | — | — |
| `getConversationSummary` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}` | 读取会话摘要 / Get conversation summary | — | ConversationSummary |
| `getConversationContextUsage` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/context-usage` | 读取上下文占用 / Get context usage | — | ConversationContextUsage |
| `listConversations` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations` | 分页查询会话 / List conversations | — | ConversationSummaryList |
| `listActiveConversations` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/active` | 查询活动会话 / List active conversations | — | ConversationIds |
| `listUnreadConversations` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/unread` | 查询未读会话 / List unread conversations | — | ConversationIds |
| `queryConversationActivities` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversations/activity/query` | 批量查询会话活动 / Query conversation activities | ConversationActivityBatchInput | ConversationActivityList |
| `markConversationRead` | `PUT /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/read-receipt` | 推进会话已读游标 / Mark a conversation as read | ConversationReadReceiptInput | ConversationReadReceipt |
| `getConversationStats` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/stats` | 读取会话统计 / Get conversation statistics | — | ConversationStats |
| `getConversationTitle` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/title` | 读取会话标题 / Get conversation title | — | ConversationTitle |
| `updateConversationTitle` | `PATCH /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/title` | 更新会话标题 / Update conversation title | ConversationTitleInput | ConversationMetadata |
| `updateConversationStatus` | `PATCH /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/status` | 更新会话状态 / Update conversation status | ConversationStatusInput | — |
| `listConversationGroups` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversation-groups` | 查询会话分组 / List conversation groups | — | ConversationGroupList |
| `createConversationGroup` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/conversation-groups` | 创建会话分组 / Create conversation group | ConversationGroupNameInput | ConversationGroup |
| `renameConversationGroup` | `PATCH /api/agents/channel/openapi/v1/{channelId}/chat/conversation-groups/{groupId}` | 重命名会话分组 / Rename conversation group | ConversationGroupNameInput | ConversationGroup |
| `deleteConversationGroup` | `DELETE /api/agents/channel/openapi/v1/{channelId}/chat/conversation-groups/{groupId}` | 删除会话分组 / Delete conversation group | — | ConversationGroupDeleted |
| `moveConversationToGroup` | `PATCH /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/group` | 移动会话分组 / Move conversation to group | UpdateConversationGroupInput | ConversationMetadata |


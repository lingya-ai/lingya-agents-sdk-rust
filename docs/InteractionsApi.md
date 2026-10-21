# InteractionsApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `approvePlan` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/plan/approve` | 提交计划审批 / Submit plan approval | PlanApprovalInput | OperationResult |
| `getPlanStatus` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/plan/{planId}/status` | 查询计划审批状态 / Get plan approval status | — | PlanStatus |
| `getUserInputStatus` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/user-input/{questionId}/status` | 查询用户问答状态 / Get user-input status | — | UserInputStatus |
| `answerUserInput` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/user-input/answer` | 提交用户回答 / Submit a user answer | UserInputAnswerInput | OperationResult |
| `getMessagePlanStatus` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages/{messageId}/plan/status` | 查询消息计划审批状态 / Get message plan status | — | PlanStatus |


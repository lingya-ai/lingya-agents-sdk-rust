# WorkspaceApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `listWorkspaceArtifacts` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/workspace/files` | 分页查询工作区制品 / List workspace artifacts | — | WorkspaceArtifactList |
| `getWorkspaceFilePreview` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/workspace/files/preview` | 创建工作区文件预览地址 / Create a workspace file preview URL | — | PreSignedReadUrl |


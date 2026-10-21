# FilesApi HTTP 契约参考 / HTTP contract reference

本页从契约操作清单生成。operationId 是跨语言契约标识，表格不是 SDK 方法签名；业务调用方式见 [SDK README](../README.md)，请求选项、认证、下载及订阅规则见 [生命周期说明](request-lifecycle.md)。channelId 由 SDK 根客户端绑定，无需逐次传入。

Generated from the operation manifest. Operation IDs identify the wire contract, not language-specific call signatures. Use the SDK README and lifecycle guide for high-level calls; channelId is bound at construction.

| operationId | HTTP | 说明 / Summary | Body schema | Response |
| --- | --- | --- | --- | --- |
| `createPreSignedUpload` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/files/pre-signed-url/write` | 创建预签名上传地址 / Create a presigned upload URL | GeneratePreSignedUrlInput | GeneratePreSignedUrlOutput |
| `confirmPreSignedUpload` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/files/pre-signed-url/confirm` | 确认预签名上传 / Confirm a presigned upload | ConfirmUploadInput | AgentFile |
| `createFileByContentMd5` | `POST /api/agents/channel/openapi/v1/{channelId}/chat/files/contentMd5` | 按 MD5 复用文件 / Reuse a file by MD5 | CreateFileInput | AgentFile |
| `fileExistsByContentMd5` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/files/meta/contentMd5` | 检查 MD5 文件是否存在 / Check file existence by MD5 | — | FileExists |
| `getConversationFilePreview` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/files/{fileId}/preview` | 创建会话文件预览地址 / Create a conversation file preview URL | — | PreSignedReadUrl |
| `getPlanIntermediateFilePreview` | `GET /api/agents/channel/openapi/v1/{channelId}/chat/conversations/{conversationId}/messages/{messageId}/plan-intermediate-files/{fileId}/preview` | 创建计划快照预览地址 / Create a plan snapshot preview URL | — | PreSignedReadUrl |


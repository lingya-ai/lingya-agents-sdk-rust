# ConversationUserMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | 字段 text / text field。 |
**multimodal_attachments** | Option<[**Vec<models::MultimodalMediaAttachment>**](MultimodalMediaAttachment.md)> | 字段 multimodalAttachments / multimodal attachments field。 | [optional]
**attachments** | Option<[**Vec<models::MediaAttachment>**](MediaAttachment.md)> | 字段 attachments / attachments field。 | [optional]
**execution_type** | **String** | 字段 executionType / execution type field。 |
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: USER) |
**metadata** | **std::collections::HashMap<String, serde_json::Value>** | 消息来源定义的动态 JSON 元数据。 / Dynamic JSON metadata defined by the message source. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# UserChatMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: USER) | 
**text** | **String** | 字段 text / text field。 | 
**multimodal_attachments** | Option<[**Vec<models::MultimodalMediaAttachment>**](MultimodalMediaAttachment.md)> | 字段 multimodalAttachments / multimodal attachments field。 | [optional]
**attachments** | Option<[**Vec<models::MediaAttachment>**](MediaAttachment.md)> | 字段 attachments / attachments field。 | [optional]
**metadata_raw_json** | Option<**String**> | 字段 metadataRawJson / metadata raw json field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



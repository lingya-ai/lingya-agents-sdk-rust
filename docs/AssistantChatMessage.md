# AssistantChatMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: ASSISTANT) | 
**text** | Option<**String**> | 字段 text / text field。 | [optional]
**reasoning_text** | Option<**String**> | 字段 reasoningText / reasoning text field。 | [optional]
**tool_calls** | [**Vec<models::ToolCall>**](ToolCall.md) | 字段 toolCalls / tool calls field。 | 
**metadata_raw_json** | Option<**String**> | 字段 metadataRawJson / metadata raw json field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# AiChatToolExecutionBriefEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: tool-execution) | 
**tool_id** | **String** | 字段 toolId / tool id field。 | 
**tool_name** | **String** | 字段 toolName / tool name field。 | 
**status** | **String** | 当前状态 / current status。 | 
**action** | **String** | 字段 action / action field。 | 
**summary** | Option<**String**> | 字段 summary / summary field。 | [optional]
**extension** | Option<[**models::ToolExtension**](ToolExtension.md)> | 字段 extension / extension field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



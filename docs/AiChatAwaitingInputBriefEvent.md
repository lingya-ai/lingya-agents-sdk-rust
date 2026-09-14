# AiChatAwaitingInputBriefEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: tool-execution-awaiting-user-input) | 
**tool_call_id** | **String** | 字段 toolCallId / tool call id field。 | 
**tool_name** | **String** | 字段 toolName / tool name field。 | 
**question_id** | **String** | 字段 questionId / question id field。 | 
**question** | **String** | 字段 question / question field。 | 
**options** | [**Vec<models::AskUserQuestionOption>**](AskUserQuestionOption.md) | 字段 options / options field。 | 
**multiple** | **bool** | 字段 multiple / multiple field。 | 
**server_now** | **chrono::DateTime<chrono::FixedOffset>** | 字段 serverNow / server now field。 | 
**timeout_seconds** | **i64** | 字段 timeoutSeconds / timeout seconds field。 | 
**question_details** | Option<**String**> | 字段 questionDetails / question details field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



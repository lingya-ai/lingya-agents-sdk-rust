# ConversationMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_id** | **String** | 消息 ID / message ID。 | 
**user_message** | [**models::ConversationUserMessage**](ConversationUserMessage.md) | 字段 userMessage / user message field。 | 
**input_tokens** | **i64** | 输入 Token 数 / input token count。 | 
**output_tokens** | **i64** | 输出 Token 数 / output token count。 | 
**total_tokens** | **i64** | 总 Token 数 / total token count。 | 
**usage** | [**models::Usage**](Usage.md) | 字段 usage / usage field。 | 
**execution_time_millis** | **i64** | 字段 executionTimeMillis / execution time millis field。 | 
**processing_steps** | **i32** | 字段 processingSteps / processing steps field。 | 
**total_tool_calls** | **i32** | 字段 totalToolCalls / total tool calls field。 | 
**status** | **String** | 当前状态 / current status。 | 
**created_time** | **chrono::DateTime<chrono::FixedOffset>** | 创建时间 / creation time。 | 
**last_update_time** | **chrono::DateTime<chrono::FixedOffset>** | 最后更新时间 / last update time。 | 
**execution_type** | **String** | 字段 executionType / execution type field。 | 
**parent_message_id** | Option<**String**> | 字段 parentMessageId / parent message id field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



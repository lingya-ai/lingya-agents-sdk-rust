# ConversationSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | **String** | 会话 ID / conversation ID。 | 
**conversation_type** | **String** | 字段 conversationType / conversation type field。 | 
**title** | Option<**String**> | 标题 / title。 | [optional]
**title_state** | **String** | 字段 titleState / title state field。 | 
**status** | **String** | 当前状态 / current status。 | 
**message_count** | **i64** | 字段 messageCount / message count field。 | 
**total_tokens** | **i64** | 总 Token 数 / total token count。 | 
**total_input_tokens** | **i64** | 字段 totalInputTokens / total input tokens field。 | 
**total_output_tokens** | **i64** | 字段 totalOutputTokens / total output tokens field。 | 
**usage** | [**models::Usage**](Usage.md) | 字段 usage / usage field。 | 
**created_time** | **chrono::DateTime<chrono::FixedOffset>** | 创建时间 / creation time。 | 
**last_update_time** | **chrono::DateTime<chrono::FixedOffset>** | 最后更新时间 / last update time。 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



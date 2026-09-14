# AsyncTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** | 异步任务 ID / asynchronous task ID。 | 
**task_type** | **String** | 字段 taskType / task type field。 | 
**title** | **String** | 标题 / title。 | 
**status** | **String** | 当前状态 / current status。 | 
**progress_percent** | Option<**i32**> | 字段 progressPercent / progress percent field。 | [optional]
**phase** | Option<**String**> | 字段 phase / phase field。 | [optional]
**status_message** | Option<**String**> | 字段 statusMessage / status message field。 | [optional]
**result_available** | **bool** | 字段 resultAvailable / result available field。 | 
**cancellable** | **bool** | 字段 cancellable / cancellable field。 | 
**failure_code** | Option<**String**> | 字段 failureCode / failure code field。 | [optional]
**failure_message** | Option<**String**> | 字段 failureMessage / failure message field。 | [optional]
**origin_conversation_id** | **String** | 字段 originConversationId / origin conversation id field。 | 
**origin_message_id** | **String** | 字段 originMessageId / origin message id field。 | 
**origin_tool_id** | **String** | 字段 originToolId / origin tool id field。 | 
**origin_tool_name** | **String** | 字段 originToolName / origin tool name field。 | 
**target_message_id** | **String** | 字段 targetMessageId / target message id field。 | 
**notification_status** | **String** | 字段 notificationStatus / notification status field。 | 
**notification_message_id** | Option<**String**> | 字段 notificationMessageId / notification message id field。 | [optional]
**created_time** | **chrono::DateTime<chrono::FixedOffset>** | 创建时间 / creation time。 | 
**started_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 字段 startedTime / started time field。 | [optional]
**completed_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 字段 completedTime / completed time field。 | [optional]
**last_update_time** | **chrono::DateTime<chrono::FixedOffset>** | 最后更新时间 / last update time。 | 
**tracking_status** | **String** | 字段 trackingStatus / tracking status field。 | 
**tracking_failure_code** | Option<**String**> | 字段 trackingFailureCode / tracking failure code field。 | [optional]
**last_poll_error** | Option<**String**> | 字段 lastPollError / last poll error field。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



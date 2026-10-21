# SubagentTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** | 异步任务 ID / asynchronous task ID。 |
**description** | **String** | 可读说明 / human-readable description。 |
**subagent_type** | **String** | 字段 subagentType / subagent type field。 |
**status** | **String** | 当前状态 / current status。 |
**phase** | Option<**String**> | 字段 phase / phase field。 | [optional]
**progress_percent** | Option<**i32**> | 字段 progressPercent / progress percent field。 | [optional]
**parent_conversation_id** | **String** | 字段 parentConversationId / parent conversation id field。 |
**parent_message_id** | **String** | 字段 parentMessageId / parent message id field。 |
**child_conversation_id** | Option<**String**> | 字段 childConversationId / child conversation id field。 | [optional]
**child_message_id** | Option<**String**> | 字段 childMessageId / child message id field。 | [optional]
**result_available** | **bool** | 字段 resultAvailable / result available field。 |
**artifact_count** | **i32** | 字段 artifactCount / artifact count field。 |
**notification_status** | **String** | 字段 notificationStatus / notification status field。 |
**failure_code** | Option<**String**> | 字段 failureCode / failure code field。 | [optional]
**failure_message** | Option<**String**> | 字段 failureMessage / failure message field。 | [optional]
**created_time** | **chrono::DateTime<chrono::FixedOffset>** | 创建时间 / creation time。 |
**started_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 字段 startedTime / started time field。 | [optional]
**completed_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 字段 completedTime / completed time field。 | [optional]
**last_update_time** | **chrono::DateTime<chrono::FixedOffset>** | 最后更新时间 / last update time。 |
**cancellable** | **bool** | 字段 cancellable / cancellable field。 |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



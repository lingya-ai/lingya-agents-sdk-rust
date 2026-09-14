# AiChatEndBriefEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: end) | 
**execution_time_millis** | **i64** | 字段 executionTimeMillis / execution time millis field。 | 
**total_usage** | [**models::Usage**](Usage.md) | 字段 totalUsage / total usage field。 | 
**message_context_usage_ratio** | Option<**f64**> | 字段 messageContextUsageRatio / message context usage ratio field。 | [optional]
**context_window_usage** | Option<[**models::ConversationContextUsage**](ConversationContextUsage.md)> | 字段 contextWindowUsage / context window usage field。 | [optional]
**artifacts** | [**Vec<models::ArtifactInfo>**](ArtifactInfo.md) | 字段 artifacts / artifacts field。 | 
**non_file_artifacts** | [**Vec<models::WorkspaceNonFileArtifact>**](WorkspaceNonFileArtifact.md) | 字段 nonFileArtifacts / non file artifacts field。 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



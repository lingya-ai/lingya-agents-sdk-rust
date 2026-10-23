# AiChatUserQueryBriefEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: user-query) |
**query** | **String** | 用户问题 / user query。 |
**attachments** | Option<[**Vec<models::MediaAttachment>**](MediaAttachment.md)> | 字段 attachments / attachments field。 | [optional]
**execution_type** | Option<**ExecutionType**> | 字段 executionType / execution type field。 (enum: USER_MESSAGE, ASYNC_TASK_NOTIFICATION, SUBAGENT_ASYNC_EXECUTION, SUBAGENT, CONTEXT_COMPRESSION, CONTEXT_INSPECTION, COLLABORATION_TURN, COLLABORATION_STEER, COLLABORATION_NOTIFICATION, GOAL_CONTINUATION) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



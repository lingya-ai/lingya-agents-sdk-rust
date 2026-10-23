# AiChatErrorBriefEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: error) |
**message** | **String** | 消息正文 / message text。 |
**terminal_reason** | Option<**TerminalReason**> | 字段 terminalReason / terminal reason field。 (enum: COMPLETED, EXECUTION_ERROR, INTERRUPTED, USER_INTERRUPTED, SYSTEM_INTERRUPTED, WORKSPACE_COMMIT_FAILED, EXECUTION_LEASE_EXPIRED, EXECUTION_DEADLINE_EXCEEDED, TERMINAL_EVENT_RECONCILED, TERMINAL_EVENT_MISSING_RECONCILED, LEGACY_STATE_RECONCILED) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



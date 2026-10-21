# AgentGoal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**goal_id** | **String** | 字段 goalId / goal id field。 |
**goal_revision** | **i32** | 字段 goalRevision / goal revision field。 |
**objective** | **String** | 字段 objective / objective field。 |
**phase** | **Phase** | 字段 phase / phase field。 (enum: ACTIVE, PAUSED, BLOCKED, COMPLETE) |
**record_state** | **RecordState** | 字段 recordState / record state field。 (enum: CURRENT, CLEARED, REPLACED) |
**activation_state** | **ActivationState** | 字段 activationState / activation state field。 (enum: ARMED, DISARMED) |
**effectively_armed** | **bool** | 字段 effectivelyArmed / effectively armed field。 |
**blocked_code** | Option<**BlockedCode**> | 字段 blockedCode / blocked code field。 (enum: ROUND_LIMIT, STEP_LIMIT, ARTIFACT_COMMIT_FAILED, HUMAN_INPUT_REQUIRED, QUEUE_FAILED, PROMPT_REJECTED, AUTH_REVOKED, STORAGE_UNAVAILABLE, EXECUTION_FAILED, MODEL_REPORTED) |
**blocked_message** | Option<**String**> | 字段 blockedMessage / blocked message field。 |
**completion_message** | Option<**String**> | 字段 completionMessage / completion message field。 |
**status_version** | **i64** | 字段 statusVersion / status version field。 |
**next_execution_due_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 字段 nextExecutionDueTime / next execution due time field。 |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



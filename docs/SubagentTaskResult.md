# SubagentTaskResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task** | [**models::SubagentTask**](SubagentTask.md) | 字段 task / task field。 |
**result_text** | Option<**String**> | 字段 resultText / result text field。 | [optional]
**artifacts** | [**Vec<models::SubagentTaskArtifact>**](SubagentTaskArtifact.md) | 字段 artifacts / artifacts field。 |
**input_tokens** | **i64** | 输入 Token 数 / input token count。 |
**output_tokens** | **i64** | 输出 Token 数 / output token count。 |
**non_file_artifacts** | [**Vec<models::WorkspaceNonFileArtifact>**](WorkspaceNonFileArtifact.md) | 字段 nonFileArtifacts / non file artifacts field。 |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# SubagentTaskArtifact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifact_id** | **String** | 字段 artifactId / artifact id field。 |
**name** | **String** | 字段 name / name field。 |
**mime_type** | Option<**String**> | MIME 类型 / MIME type。 | [optional]
**size** | Option<**i64**> | 大小（字节）或分页容量 / byte size or page size。 | [optional]
**relative_path** | Option<**String**> | 字段 relativePath / relative path field。 | [optional]
**delivery_status** | **DeliveryStatus** | 字段 deliveryStatus / delivery status field。 (enum: NOT_APPLICABLE, UNVALIDATED, ACCEPTED, ACCEPTED_WITH_WARNINGS, BLOCKED) |
**issue_codes** | **Vec<String>** | 字段 issueCodes / issue codes field。 |
**recoverable** | **bool** | 字段 recoverable / recoverable field。 |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



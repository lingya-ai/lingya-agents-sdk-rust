# OpenApiToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | 一次性令牌原文，不得写入日志。 / Single-use token; never log its value. |
**expires_at** | **chrono::DateTime<chrono::FixedOffset>** | 未消费令牌的截止时间，不代表截止前可以重复使用。 / Deadline for an unused token, not a reusable session lifetime. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



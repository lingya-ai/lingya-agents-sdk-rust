# AiChatInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | 用户问题 / user query。 |
**conversation_id** | Option<**String**> | 会话 ID / conversation ID。 | [optional]
**chat_model_spec** | Option<[**models::ChatModelSpec**](ChatModelSpec.md)> | 字段 chatModelSpec / chat model spec field。 | [optional]
**files** | Option<[**Vec<models::AiChatFileRef>**](AiChatFileRef.md)> | 字段 files / files field。 | [optional]
**plan_mode** | Option<**bool**> | 计划模式；省略时沿用会话设置。 / Plan mode; omitted values preserve the conversation setting. | [optional]
**goal_mode** | Option<**bool**> | 创建新目标；不能与计划模式同时开启，目标正文最多 4000 字符。 / Create a goal; cannot be enabled together with plan mode, and the objective is limited to 4000 characters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



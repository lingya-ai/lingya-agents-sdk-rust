# GeneratePreSignedUrlOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**support** | **bool** | 当前存储是否支持该操作 / whether the storage supports this operation。 |
**file_uk** | Option<**String**> | 字段 fileUk / file uk field。 | [optional]
**url** | Option<**String**> | 预签名 URL / presigned URL。 | [optional]
**upload_method** | Option<**UploadMethod**> | 客户端上传方式 / upload method。 (enum: PUT, POST_FORM) | [optional]
**form_fields** | Option<**std::collections::HashMap<String, String>**> | 表单上传字段 / form fields。 | [optional]
**maximum_bytes** | Option<**i64**> | 受限上传允许的最大字节数 / maximum bytes for bounded upload。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



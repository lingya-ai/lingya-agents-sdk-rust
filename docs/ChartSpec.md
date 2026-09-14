# ChartSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **i32** | 字段 version / version field。 | 
**r#type** | **Type** | 类型判别值 / type discriminator。 (enum: BAR, LINE, BAR_LINE, STACKED_BAR, PIE) | 
**title** | **String** | 标题 / title。 | 
**subtitle** | Option<**String**> | 字段 subtitle / subtitle field。 | [optional]
**category_column** | **String** | 字段 categoryColumn / category column field。 | 
**category_type** | **CategoryType** | 字段 categoryType / category type field。 (enum: CATEGORY, TIME) | 
**orientation** | **Orientation** | 字段 orientation / orientation field。 (enum: VERTICAL, HORIZONTAL) | 
**series** | [**Vec<models::ChartSeries>**](ChartSeries.md) | 字段 series / series field。 | 
**legend** | **bool** | 字段 legend / legend field。 | 
**data_zoom** | **DataZoom** | 字段 dataZoom / data zoom field。 (enum: NONE, AUTO) | 
**null_policy** | **NullPolicy** | 字段 nullPolicy / null policy field。 (enum: GAP, ZERO, REJECT) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Metric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Metric identifier, unique within a product. | 
**kind** | **Kind** |  (enum: number, text, status, progress, gauge, list, timeseries, bar, table) | 
**label** | Option<**String**> |  | [optional]
**format** | Option<[**models::MetricFormat**](MetricFormat.md)> |  | [optional]
**value** | Option<**serde_json::Value**> | Kind-specific value; see the Metric description. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



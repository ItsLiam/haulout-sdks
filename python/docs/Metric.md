# Metric

One current metric value. The value shape depends on kind and is validated by the API: number: a JSON number; text: a string up to 4096 characters; status: {state: neutral|positive|warning|critical, text}; progress: {current >= 0, total > 0}; gauge: {value, min, max}; list: {items: [{id, label, detail?, done?, progress?, state?}]}, up to 500 items; timeseries: {series: [{label, color?, points: [{t, v}]}]}, 1–8 series of up to 2000 points each; t is an RFC 3339 timestamp and v is a number or null (a gap); bar: {bars: [{label, value, color?}]}, 1–100 bars; table: {columns: [label], rows: [[scalar]]}, 1–16 columns and up to 500 rows.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Metric identifier, unique within a product. | 
**kind** | **str** |  | 
**label** | **str** |  | [optional] 
**format** | [**MetricFormat**](MetricFormat.md) |  | [optional] 
**value** | **object** | Kind-specific value; see the Metric description. | 

## Example

```python
from haulout.models.metric import Metric

# TODO update the JSON string below
json = "{}"
# create an instance of Metric from a JSON string
metric_instance = Metric.from_json(json)
# print the JSON string representation of the object
print(Metric.to_json())

# convert the object into a dict
metric_dict = metric_instance.to_dict()
# create an instance of Metric from a dict
metric_from_dict = Metric.from_dict(metric_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



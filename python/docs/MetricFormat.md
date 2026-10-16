# MetricFormat

Display hint for a metric value.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**style** | **str** |  | 
**precision** | **int** |  | [optional] 
**unit** | **str** |  | [optional] 

## Example

```python
from haulout.models.metric_format import MetricFormat

# TODO update the JSON string below
json = "{}"
# create an instance of MetricFormat from a JSON string
metric_format_instance = MetricFormat.from_json(json)
# print the JSON string representation of the object
print(MetricFormat.to_json())

# convert the object into a dict
metric_format_dict = metric_format_instance.to_dict()
# create an instance of MetricFormat from a dict
metric_format_from_dict = MetricFormat.from_dict(metric_format_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



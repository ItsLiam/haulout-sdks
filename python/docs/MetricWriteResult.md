# MetricWriteResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Metric identifier, unique within a product. | 
**revision** | **int** |  | 
**updated_at** | **datetime** |  | 

## Example

```python
from haulout.models.metric_write_result import MetricWriteResult

# TODO update the JSON string below
json = "{}"
# create an instance of MetricWriteResult from a JSON string
metric_write_result_instance = MetricWriteResult.from_json(json)
# print the JSON string representation of the object
print(MetricWriteResult.to_json())

# convert the object into a dict
metric_write_result_dict = metric_write_result_instance.to_dict()
# create an instance of MetricWriteResult from a dict
metric_write_result_from_dict = MetricWriteResult.from_dict(metric_write_result_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



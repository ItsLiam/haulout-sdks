# DeletedMetric


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Metric identifier, unique within a product. | 

## Example

```python
from haulout.models.deleted_metric import DeletedMetric

# TODO update the JSON string below
json = "{}"
# create an instance of DeletedMetric from a JSON string
deleted_metric_instance = DeletedMetric.from_json(json)
# print the JSON string representation of the object
print(DeletedMetric.to_json())

# convert the object into a dict
deleted_metric_dict = deleted_metric_instance.to_dict()
# create an instance of DeletedMetric from a dict
deleted_metric_from_dict = DeletedMetric.from_dict(deleted_metric_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



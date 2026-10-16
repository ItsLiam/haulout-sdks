# DeleteMetricsByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**p_keys** | **List[str]** |  | 

## Example

```python
from haulout.models.delete_metrics_by_key_request import DeleteMetricsByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of DeleteMetricsByKeyRequest from a JSON string
delete_metrics_by_key_request_instance = DeleteMetricsByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(DeleteMetricsByKeyRequest.to_json())

# convert the object into a dict
delete_metrics_by_key_request_dict = delete_metrics_by_key_request_instance.to_dict()
# create an instance of DeleteMetricsByKeyRequest from a dict
delete_metrics_by_key_request_from_dict = DeleteMetricsByKeyRequest.from_dict(delete_metrics_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



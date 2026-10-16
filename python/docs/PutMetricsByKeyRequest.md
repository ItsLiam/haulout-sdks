# PutMetricsByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**p_metrics** | [**List[Metric]**](Metric.md) |  | 

## Example

```python
from haulout.models.put_metrics_by_key_request import PutMetricsByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of PutMetricsByKeyRequest from a JSON string
put_metrics_by_key_request_instance = PutMetricsByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(PutMetricsByKeyRequest.to_json())

# convert the object into a dict
put_metrics_by_key_request_dict = put_metrics_by_key_request_instance.to_dict()
# create an instance of PutMetricsByKeyRequest from a dict
put_metrics_by_key_request_from_dict = PutMetricsByKeyRequest.from_dict(put_metrics_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



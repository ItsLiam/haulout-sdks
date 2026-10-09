# PublishEventBatchByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**p_idempotency_key** | **UUID** |  | 
**p_definition_revision** | **int** |  | 
**p_events** | [**List[PublishEventBatchByKeyRequestPEventsInner]**](PublishEventBatchByKeyRequestPEventsInner.md) |  | 

## Example

```python
from haulout.models.publish_event_batch_by_key_request import PublishEventBatchByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of PublishEventBatchByKeyRequest from a JSON string
publish_event_batch_by_key_request_instance = PublishEventBatchByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(PublishEventBatchByKeyRequest.to_json())

# convert the object into a dict
publish_event_batch_by_key_request_dict = publish_event_batch_by_key_request_instance.to_dict()
# create an instance of PublishEventBatchByKeyRequest from a dict
publish_event_batch_by_key_request_from_dict = PublishEventBatchByKeyRequest.from_dict(publish_event_batch_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# PublishEventBatchByKeyRequestPEventsInner


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**source** | **str** |  | 
**time** | **str** | RFC 3339 timestamp string. | 
**fields** | **Dict[str, object]** | Fields validated against the selected stream declaration; values preserve arbitrary JSON. | 

## Example

```python
from haulout.models.publish_event_batch_by_key_request_p_events_inner import PublishEventBatchByKeyRequestPEventsInner

# TODO update the JSON string below
json = "{}"
# create an instance of PublishEventBatchByKeyRequestPEventsInner from a JSON string
publish_event_batch_by_key_request_p_events_inner_instance = PublishEventBatchByKeyRequestPEventsInner.from_json(json)
# print the JSON string representation of the object
print(PublishEventBatchByKeyRequestPEventsInner.to_json())

# convert the object into a dict
publish_event_batch_by_key_request_p_events_inner_dict = publish_event_batch_by_key_request_p_events_inner_instance.to_dict()
# create an instance of PublishEventBatchByKeyRequestPEventsInner from a dict
publish_event_batch_by_key_request_p_events_inner_from_dict = PublishEventBatchByKeyRequestPEventsInner.from_dict(publish_event_batch_by_key_request_p_events_inner_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



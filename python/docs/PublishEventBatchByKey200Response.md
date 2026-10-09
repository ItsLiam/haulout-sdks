# PublishEventBatchByKey200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted** | **int** |  | 
**duplicates** | **int** |  | 
**cursor** | **str** | Cursor encoded as a decimal string. | 
**committed_at** | **datetime** |  | 
**dedupe_until** | **datetime** |  | 

## Example

```python
from haulout.models.publish_event_batch_by_key200_response import PublishEventBatchByKey200Response

# TODO update the JSON string below
json = "{}"
# create an instance of PublishEventBatchByKey200Response from a JSON string
publish_event_batch_by_key200_response_instance = PublishEventBatchByKey200Response.from_json(json)
# print the JSON string representation of the object
print(PublishEventBatchByKey200Response.to_json())

# convert the object into a dict
publish_event_batch_by_key200_response_dict = publish_event_batch_by_key200_response_instance.to_dict()
# create an instance of PublishEventBatchByKey200Response from a dict
publish_event_batch_by_key200_response_from_dict = PublishEventBatchByKey200Response.from_dict(publish_event_batch_by_key200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



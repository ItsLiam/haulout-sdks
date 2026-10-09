# PostgrestError

PostgREST JSON error body; SQLSTATE is in code and text in message.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **str** |  | 
**message** | **str** |  | 
**details** | **str** |  | [optional] 
**hint** | **str** |  | [optional] 

## Example

```python
from haulout.models.postgrest_error import PostgrestError

# TODO update the JSON string below
json = "{}"
# create an instance of PostgrestError from a JSON string
postgrest_error_instance = PostgrestError.from_json(json)
# print the JSON string representation of the object
print(PostgrestError.to_json())

# convert the object into a dict
postgrest_error_dict = postgrest_error_instance.to_dict()
# create an instance of PostgrestError from a dict
postgrest_error_from_dict = PostgrestError.from_dict(postgrest_error_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



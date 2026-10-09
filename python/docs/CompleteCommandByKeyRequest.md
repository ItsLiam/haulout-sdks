# CompleteCommandByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**p_command_id** | **UUID** |  | 
**p_lease_id** | **UUID** |  | 
**p_status** | **str** |  | 
**p_result** | **object** | Arbitrary JSON result value, including null. | 

## Example

```python
from haulout.models.complete_command_by_key_request import CompleteCommandByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CompleteCommandByKeyRequest from a JSON string
complete_command_by_key_request_instance = CompleteCommandByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(CompleteCommandByKeyRequest.to_json())

# convert the object into a dict
complete_command_by_key_request_dict = complete_command_by_key_request_instance.to_dict()
# create an instance of CompleteCommandByKeyRequest from a dict
complete_command_by_key_request_from_dict = CompleteCommandByKeyRequest.from_dict(complete_command_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



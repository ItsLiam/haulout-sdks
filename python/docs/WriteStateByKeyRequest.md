# WriteStateByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**p_expected_revision** | **int** |  | 
**p_values** | **Dict[str, object]** | Bounded values object; properties and their values are validated against declared HSP state fields. Values may be nested objects where the declaration uses dotted paths. | 

## Example

```python
from haulout.models.write_state_by_key_request import WriteStateByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of WriteStateByKeyRequest from a JSON string
write_state_by_key_request_instance = WriteStateByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(WriteStateByKeyRequest.to_json())

# convert the object into a dict
write_state_by_key_request_dict = write_state_by_key_request_instance.to_dict()
# create an instance of WriteStateByKeyRequest from a dict
write_state_by_key_request_from_dict = WriteStateByKeyRequest.from_dict(write_state_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



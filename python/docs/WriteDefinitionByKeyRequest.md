# WriteDefinitionByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**p_expected_revision** | **int** |  | 
**p_definition** | [**ProductDefinition**](ProductDefinition.md) |  | 

## Example

```python
from haulout.models.write_definition_by_key_request import WriteDefinitionByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of WriteDefinitionByKeyRequest from a JSON string
write_definition_by_key_request_instance = WriteDefinitionByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(WriteDefinitionByKeyRequest.to_json())

# convert the object into a dict
write_definition_by_key_request_dict = write_definition_by_key_request_instance.to_dict()
# create an instance of WriteDefinitionByKeyRequest from a dict
write_definition_by_key_request_from_dict = WriteDefinitionByKeyRequest.from_dict(write_definition_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



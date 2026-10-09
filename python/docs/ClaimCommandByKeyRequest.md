# ClaimCommandByKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p_product_key** | **str** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**p_worker_id** | **str** |  | 
**p_claim_id** | **UUID** |  | 

## Example

```python
from haulout.models.claim_command_by_key_request import ClaimCommandByKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ClaimCommandByKeyRequest from a JSON string
claim_command_by_key_request_instance = ClaimCommandByKeyRequest.from_json(json)
# print the JSON string representation of the object
print(ClaimCommandByKeyRequest.to_json())

# convert the object into a dict
claim_command_by_key_request_dict = claim_command_by_key_request_instance.to_dict()
# create an instance of ClaimCommandByKeyRequest from a dict
claim_command_by_key_request_from_dict = ClaimCommandByKeyRequest.from_dict(claim_command_by_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



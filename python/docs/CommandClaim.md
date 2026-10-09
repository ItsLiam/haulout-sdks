# CommandClaim


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command_id** | **UUID** |  | 
**command** | **str** |  | 
**arguments** | **Dict[str, object]** |  | 
**definition_revision** | **int** |  | 
**lease_id** | **UUID** |  | 
**lease_expires_at** | **datetime** |  | 

## Example

```python
from haulout.models.command_claim import CommandClaim

# TODO update the JSON string below
json = "{}"
# create an instance of CommandClaim from a JSON string
command_claim_instance = CommandClaim.from_json(json)
# print the JSON string representation of the object
print(CommandClaim.to_json())

# convert the object into a dict
command_claim_dict = command_claim_instance.to_dict()
# create an instance of CommandClaim from a dict
command_claim_from_dict = CommandClaim.from_dict(command_claim_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



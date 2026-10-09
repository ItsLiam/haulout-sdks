# ModelField

HSP field declaration; SQL performs authoritative validation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **str** |  | 
**required** | **bool** |  | [optional] 
**unit** | **str** |  | [optional] 
**minimum** | **float** |  | [optional] 
**maximum** | **float** |  | [optional] 
**max_length** | **int** |  | [optional] 
**enum** | **List[object]** |  | [optional] 

## Example

```python
from haulout.models.model_field import ModelField

# TODO update the JSON string below
json = "{}"
# create an instance of ModelField from a JSON string
model_field_instance = ModelField.from_json(json)
# print the JSON string representation of the object
print(ModelField.to_json())

# convert the object into a dict
model_field_dict = model_field_instance.to_dict()
# create an instance of ModelField from a dict
model_field_from_dict = ModelField.from_dict(model_field_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



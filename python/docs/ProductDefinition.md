# ProductDefinition

HSP ProductDefinition v2.0. The API is authoritative for recursive dashboard and component validation; this DTO keeps dashboard contents as opaque JSON.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **str** |  | 
**version** | **str** |  | 
**streams** | [**Dict[str, Stream]**](Stream.md) | Map of declared stream names to HSP stream descriptions. | 
**state** | [**Dict[str, ModelField]**](ModelField.md) | Map of declared state paths to field declarations. | 
**commands** | [**Dict[str, Command]**](Command.md) | Map of declared command names to command declarations. | 
**dashboards** | **List[Dict[str, object]]** | Ordered dashboard descriptors. Their recursively validated HSP 2.0 structures are transported as opaque JSON by this generated DTO. | 

## Example

```python
from haulout.models.product_definition import ProductDefinition

# TODO update the JSON string below
json = "{}"
# create an instance of ProductDefinition from a JSON string
product_definition_instance = ProductDefinition.from_json(json)
# print the JSON string representation of the object
print(ProductDefinition.to_json())

# convert the object into a dict
product_definition_dict = product_definition_instance.to_dict()
# create an instance of ProductDefinition from a dict
product_definition_from_dict = ProductDefinition.from_dict(product_definition_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



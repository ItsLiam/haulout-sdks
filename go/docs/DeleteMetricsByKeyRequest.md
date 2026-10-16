# DeleteMetricsByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**PKeys** | **[]string** |  | 

## Methods

### NewDeleteMetricsByKeyRequest

`func NewDeleteMetricsByKeyRequest(pProductKey string, pKeys []string, ) *DeleteMetricsByKeyRequest`

NewDeleteMetricsByKeyRequest instantiates a new DeleteMetricsByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeleteMetricsByKeyRequestWithDefaults

`func NewDeleteMetricsByKeyRequestWithDefaults() *DeleteMetricsByKeyRequest`

NewDeleteMetricsByKeyRequestWithDefaults instantiates a new DeleteMetricsByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *DeleteMetricsByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *DeleteMetricsByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *DeleteMetricsByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPKeys

`func (o *DeleteMetricsByKeyRequest) GetPKeys() []string`

GetPKeys returns the PKeys field if non-nil, zero value otherwise.

### GetPKeysOk

`func (o *DeleteMetricsByKeyRequest) GetPKeysOk() (*[]string, bool)`

GetPKeysOk returns a tuple with the PKeys field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPKeys

`func (o *DeleteMetricsByKeyRequest) SetPKeys(v []string)`

SetPKeys sets PKeys field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



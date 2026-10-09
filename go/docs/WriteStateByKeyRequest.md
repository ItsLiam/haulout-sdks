# WriteStateByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**PExpectedRevision** | **int64** |  | 
**PValues** | **map[string]interface{}** | Bounded values object; properties and their values are validated against declared HSP state fields. Values may be nested objects where the declaration uses dotted paths. | 

## Methods

### NewWriteStateByKeyRequest

`func NewWriteStateByKeyRequest(pProductKey string, pExpectedRevision int64, pValues map[string]interface{}, ) *WriteStateByKeyRequest`

NewWriteStateByKeyRequest instantiates a new WriteStateByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWriteStateByKeyRequestWithDefaults

`func NewWriteStateByKeyRequestWithDefaults() *WriteStateByKeyRequest`

NewWriteStateByKeyRequestWithDefaults instantiates a new WriteStateByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *WriteStateByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *WriteStateByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *WriteStateByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPExpectedRevision

`func (o *WriteStateByKeyRequest) GetPExpectedRevision() int64`

GetPExpectedRevision returns the PExpectedRevision field if non-nil, zero value otherwise.

### GetPExpectedRevisionOk

`func (o *WriteStateByKeyRequest) GetPExpectedRevisionOk() (*int64, bool)`

GetPExpectedRevisionOk returns a tuple with the PExpectedRevision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPExpectedRevision

`func (o *WriteStateByKeyRequest) SetPExpectedRevision(v int64)`

SetPExpectedRevision sets PExpectedRevision field to given value.


### GetPValues

`func (o *WriteStateByKeyRequest) GetPValues() map[string]interface{}`

GetPValues returns the PValues field if non-nil, zero value otherwise.

### GetPValuesOk

`func (o *WriteStateByKeyRequest) GetPValuesOk() (*map[string]interface{}, bool)`

GetPValuesOk returns a tuple with the PValues field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPValues

`func (o *WriteStateByKeyRequest) SetPValues(v map[string]interface{})`

SetPValues sets PValues field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



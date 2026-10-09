# WriteDefinitionByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**PExpectedRevision** | **int64** |  | 
**PDefinition** | [**ProductDefinition**](ProductDefinition.md) |  | 

## Methods

### NewWriteDefinitionByKeyRequest

`func NewWriteDefinitionByKeyRequest(pProductKey string, pExpectedRevision int64, pDefinition ProductDefinition, ) *WriteDefinitionByKeyRequest`

NewWriteDefinitionByKeyRequest instantiates a new WriteDefinitionByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWriteDefinitionByKeyRequestWithDefaults

`func NewWriteDefinitionByKeyRequestWithDefaults() *WriteDefinitionByKeyRequest`

NewWriteDefinitionByKeyRequestWithDefaults instantiates a new WriteDefinitionByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *WriteDefinitionByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *WriteDefinitionByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *WriteDefinitionByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPExpectedRevision

`func (o *WriteDefinitionByKeyRequest) GetPExpectedRevision() int64`

GetPExpectedRevision returns the PExpectedRevision field if non-nil, zero value otherwise.

### GetPExpectedRevisionOk

`func (o *WriteDefinitionByKeyRequest) GetPExpectedRevisionOk() (*int64, bool)`

GetPExpectedRevisionOk returns a tuple with the PExpectedRevision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPExpectedRevision

`func (o *WriteDefinitionByKeyRequest) SetPExpectedRevision(v int64)`

SetPExpectedRevision sets PExpectedRevision field to given value.


### GetPDefinition

`func (o *WriteDefinitionByKeyRequest) GetPDefinition() ProductDefinition`

GetPDefinition returns the PDefinition field if non-nil, zero value otherwise.

### GetPDefinitionOk

`func (o *WriteDefinitionByKeyRequest) GetPDefinitionOk() (*ProductDefinition, bool)`

GetPDefinitionOk returns a tuple with the PDefinition field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPDefinition

`func (o *WriteDefinitionByKeyRequest) SetPDefinition(v ProductDefinition)`

SetPDefinition sets PDefinition field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CompleteCommandByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**PCommandId** | **string** |  | 
**PLeaseId** | **string** |  | 
**PStatus** | **string** |  | 
**PResult** | **interface{}** | Arbitrary JSON result value, including null. | 

## Methods

### NewCompleteCommandByKeyRequest

`func NewCompleteCommandByKeyRequest(pProductKey string, pCommandId string, pLeaseId string, pStatus string, pResult interface{}, ) *CompleteCommandByKeyRequest`

NewCompleteCommandByKeyRequest instantiates a new CompleteCommandByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCompleteCommandByKeyRequestWithDefaults

`func NewCompleteCommandByKeyRequestWithDefaults() *CompleteCommandByKeyRequest`

NewCompleteCommandByKeyRequestWithDefaults instantiates a new CompleteCommandByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *CompleteCommandByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *CompleteCommandByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *CompleteCommandByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPCommandId

`func (o *CompleteCommandByKeyRequest) GetPCommandId() string`

GetPCommandId returns the PCommandId field if non-nil, zero value otherwise.

### GetPCommandIdOk

`func (o *CompleteCommandByKeyRequest) GetPCommandIdOk() (*string, bool)`

GetPCommandIdOk returns a tuple with the PCommandId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPCommandId

`func (o *CompleteCommandByKeyRequest) SetPCommandId(v string)`

SetPCommandId sets PCommandId field to given value.


### GetPLeaseId

`func (o *CompleteCommandByKeyRequest) GetPLeaseId() string`

GetPLeaseId returns the PLeaseId field if non-nil, zero value otherwise.

### GetPLeaseIdOk

`func (o *CompleteCommandByKeyRequest) GetPLeaseIdOk() (*string, bool)`

GetPLeaseIdOk returns a tuple with the PLeaseId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPLeaseId

`func (o *CompleteCommandByKeyRequest) SetPLeaseId(v string)`

SetPLeaseId sets PLeaseId field to given value.


### GetPStatus

`func (o *CompleteCommandByKeyRequest) GetPStatus() string`

GetPStatus returns the PStatus field if non-nil, zero value otherwise.

### GetPStatusOk

`func (o *CompleteCommandByKeyRequest) GetPStatusOk() (*string, bool)`

GetPStatusOk returns a tuple with the PStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPStatus

`func (o *CompleteCommandByKeyRequest) SetPStatus(v string)`

SetPStatus sets PStatus field to given value.


### GetPResult

`func (o *CompleteCommandByKeyRequest) GetPResult() interface{}`

GetPResult returns the PResult field if non-nil, zero value otherwise.

### GetPResultOk

`func (o *CompleteCommandByKeyRequest) GetPResultOk() (*interface{}, bool)`

GetPResultOk returns a tuple with the PResult field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPResult

`func (o *CompleteCommandByKeyRequest) SetPResult(v interface{})`

SetPResult sets PResult field to given value.


### SetPResultNil

`func (o *CompleteCommandByKeyRequest) SetPResultNil(b bool)`

 SetPResultNil sets the value for PResult to be an explicit nil

### UnsetPResult
`func (o *CompleteCommandByKeyRequest) UnsetPResult()`

UnsetPResult ensures that no value is present for PResult, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



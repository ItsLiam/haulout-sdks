# ClaimCommandByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**PWorkerId** | **string** |  | 
**PClaimId** | **string** |  | 

## Methods

### NewClaimCommandByKeyRequest

`func NewClaimCommandByKeyRequest(pProductKey string, pWorkerId string, pClaimId string, ) *ClaimCommandByKeyRequest`

NewClaimCommandByKeyRequest instantiates a new ClaimCommandByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewClaimCommandByKeyRequestWithDefaults

`func NewClaimCommandByKeyRequestWithDefaults() *ClaimCommandByKeyRequest`

NewClaimCommandByKeyRequestWithDefaults instantiates a new ClaimCommandByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *ClaimCommandByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *ClaimCommandByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *ClaimCommandByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPWorkerId

`func (o *ClaimCommandByKeyRequest) GetPWorkerId() string`

GetPWorkerId returns the PWorkerId field if non-nil, zero value otherwise.

### GetPWorkerIdOk

`func (o *ClaimCommandByKeyRequest) GetPWorkerIdOk() (*string, bool)`

GetPWorkerIdOk returns a tuple with the PWorkerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPWorkerId

`func (o *ClaimCommandByKeyRequest) SetPWorkerId(v string)`

SetPWorkerId sets PWorkerId field to given value.


### GetPClaimId

`func (o *ClaimCommandByKeyRequest) GetPClaimId() string`

GetPClaimId returns the PClaimId field if non-nil, zero value otherwise.

### GetPClaimIdOk

`func (o *ClaimCommandByKeyRequest) GetPClaimIdOk() (*string, bool)`

GetPClaimIdOk returns a tuple with the PClaimId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPClaimId

`func (o *ClaimCommandByKeyRequest) SetPClaimId(v string)`

SetPClaimId sets PClaimId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



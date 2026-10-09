# PublishEventBatchByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key (hk_&lt;test|live&gt;.&lt;product UUID&gt;.&lt;base64url secret&gt;). Supplied only in p_product_key; rotation invalidates the old key immediately. | 
**PIdempotencyKey** | **string** |  | 
**PDefinitionRevision** | **int64** |  | 
**PEvents** | [**[]PublishEventBatchByKeyRequestPEventsInner**](PublishEventBatchByKeyRequestPEventsInner.md) |  | 

## Methods

### NewPublishEventBatchByKeyRequest

`func NewPublishEventBatchByKeyRequest(pProductKey string, pIdempotencyKey string, pDefinitionRevision int64, pEvents []PublishEventBatchByKeyRequestPEventsInner, ) *PublishEventBatchByKeyRequest`

NewPublishEventBatchByKeyRequest instantiates a new PublishEventBatchByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublishEventBatchByKeyRequestWithDefaults

`func NewPublishEventBatchByKeyRequestWithDefaults() *PublishEventBatchByKeyRequest`

NewPublishEventBatchByKeyRequestWithDefaults instantiates a new PublishEventBatchByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *PublishEventBatchByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *PublishEventBatchByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *PublishEventBatchByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPIdempotencyKey

`func (o *PublishEventBatchByKeyRequest) GetPIdempotencyKey() string`

GetPIdempotencyKey returns the PIdempotencyKey field if non-nil, zero value otherwise.

### GetPIdempotencyKeyOk

`func (o *PublishEventBatchByKeyRequest) GetPIdempotencyKeyOk() (*string, bool)`

GetPIdempotencyKeyOk returns a tuple with the PIdempotencyKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPIdempotencyKey

`func (o *PublishEventBatchByKeyRequest) SetPIdempotencyKey(v string)`

SetPIdempotencyKey sets PIdempotencyKey field to given value.


### GetPDefinitionRevision

`func (o *PublishEventBatchByKeyRequest) GetPDefinitionRevision() int64`

GetPDefinitionRevision returns the PDefinitionRevision field if non-nil, zero value otherwise.

### GetPDefinitionRevisionOk

`func (o *PublishEventBatchByKeyRequest) GetPDefinitionRevisionOk() (*int64, bool)`

GetPDefinitionRevisionOk returns a tuple with the PDefinitionRevision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPDefinitionRevision

`func (o *PublishEventBatchByKeyRequest) SetPDefinitionRevision(v int64)`

SetPDefinitionRevision sets PDefinitionRevision field to given value.


### GetPEvents

`func (o *PublishEventBatchByKeyRequest) GetPEvents() []PublishEventBatchByKeyRequestPEventsInner`

GetPEvents returns the PEvents field if non-nil, zero value otherwise.

### GetPEventsOk

`func (o *PublishEventBatchByKeyRequest) GetPEventsOk() (*[]PublishEventBatchByKeyRequestPEventsInner, bool)`

GetPEventsOk returns a tuple with the PEvents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPEvents

`func (o *PublishEventBatchByKeyRequest) SetPEvents(v []PublishEventBatchByKeyRequestPEventsInner)`

SetPEvents sets PEvents field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



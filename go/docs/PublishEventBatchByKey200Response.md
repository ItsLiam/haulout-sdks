# PublishEventBatchByKey200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Accepted** | **int32** |  | 
**Duplicates** | **int32** |  | 
**Cursor** | **string** | Cursor encoded as a decimal string. | 
**CommittedAt** | **time.Time** |  | 
**DedupeUntil** | **time.Time** |  | 

## Methods

### NewPublishEventBatchByKey200Response

`func NewPublishEventBatchByKey200Response(accepted int32, duplicates int32, cursor string, committedAt time.Time, dedupeUntil time.Time, ) *PublishEventBatchByKey200Response`

NewPublishEventBatchByKey200Response instantiates a new PublishEventBatchByKey200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublishEventBatchByKey200ResponseWithDefaults

`func NewPublishEventBatchByKey200ResponseWithDefaults() *PublishEventBatchByKey200Response`

NewPublishEventBatchByKey200ResponseWithDefaults instantiates a new PublishEventBatchByKey200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccepted

`func (o *PublishEventBatchByKey200Response) GetAccepted() int32`

GetAccepted returns the Accepted field if non-nil, zero value otherwise.

### GetAcceptedOk

`func (o *PublishEventBatchByKey200Response) GetAcceptedOk() (*int32, bool)`

GetAcceptedOk returns a tuple with the Accepted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccepted

`func (o *PublishEventBatchByKey200Response) SetAccepted(v int32)`

SetAccepted sets Accepted field to given value.


### GetDuplicates

`func (o *PublishEventBatchByKey200Response) GetDuplicates() int32`

GetDuplicates returns the Duplicates field if non-nil, zero value otherwise.

### GetDuplicatesOk

`func (o *PublishEventBatchByKey200Response) GetDuplicatesOk() (*int32, bool)`

GetDuplicatesOk returns a tuple with the Duplicates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDuplicates

`func (o *PublishEventBatchByKey200Response) SetDuplicates(v int32)`

SetDuplicates sets Duplicates field to given value.


### GetCursor

`func (o *PublishEventBatchByKey200Response) GetCursor() string`

GetCursor returns the Cursor field if non-nil, zero value otherwise.

### GetCursorOk

`func (o *PublishEventBatchByKey200Response) GetCursorOk() (*string, bool)`

GetCursorOk returns a tuple with the Cursor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCursor

`func (o *PublishEventBatchByKey200Response) SetCursor(v string)`

SetCursor sets Cursor field to given value.


### GetCommittedAt

`func (o *PublishEventBatchByKey200Response) GetCommittedAt() time.Time`

GetCommittedAt returns the CommittedAt field if non-nil, zero value otherwise.

### GetCommittedAtOk

`func (o *PublishEventBatchByKey200Response) GetCommittedAtOk() (*time.Time, bool)`

GetCommittedAtOk returns a tuple with the CommittedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCommittedAt

`func (o *PublishEventBatchByKey200Response) SetCommittedAt(v time.Time)`

SetCommittedAt sets CommittedAt field to given value.


### GetDedupeUntil

`func (o *PublishEventBatchByKey200Response) GetDedupeUntil() time.Time`

GetDedupeUntil returns the DedupeUntil field if non-nil, zero value otherwise.

### GetDedupeUntilOk

`func (o *PublishEventBatchByKey200Response) GetDedupeUntilOk() (*time.Time, bool)`

GetDedupeUntilOk returns a tuple with the DedupeUntil field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDedupeUntil

`func (o *PublishEventBatchByKey200Response) SetDedupeUntil(v time.Time)`

SetDedupeUntil sets DedupeUntil field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# PublishEventBatchByKeyRequestPEventsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Source** | **string** |  | 
**Time** | **string** | RFC 3339 timestamp string. | 
**Fields** | **map[string]interface{}** | Fields validated against the selected stream declaration; values preserve arbitrary JSON. | 

## Methods

### NewPublishEventBatchByKeyRequestPEventsInner

`func NewPublishEventBatchByKeyRequestPEventsInner(id string, source string, time string, fields map[string]interface{}, ) *PublishEventBatchByKeyRequestPEventsInner`

NewPublishEventBatchByKeyRequestPEventsInner instantiates a new PublishEventBatchByKeyRequestPEventsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublishEventBatchByKeyRequestPEventsInnerWithDefaults

`func NewPublishEventBatchByKeyRequestPEventsInnerWithDefaults() *PublishEventBatchByKeyRequestPEventsInner`

NewPublishEventBatchByKeyRequestPEventsInnerWithDefaults instantiates a new PublishEventBatchByKeyRequestPEventsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PublishEventBatchByKeyRequestPEventsInner) SetId(v string)`

SetId sets Id field to given value.


### GetSource

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *PublishEventBatchByKeyRequestPEventsInner) SetSource(v string)`

SetSource sets Source field to given value.


### GetTime

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetTime() string`

GetTime returns the Time field if non-nil, zero value otherwise.

### GetTimeOk

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetTimeOk() (*string, bool)`

GetTimeOk returns a tuple with the Time field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTime

`func (o *PublishEventBatchByKeyRequestPEventsInner) SetTime(v string)`

SetTime sets Time field to given value.


### GetFields

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetFields() map[string]interface{}`

GetFields returns the Fields field if non-nil, zero value otherwise.

### GetFieldsOk

`func (o *PublishEventBatchByKeyRequestPEventsInner) GetFieldsOk() (*map[string]interface{}, bool)`

GetFieldsOk returns a tuple with the Fields field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFields

`func (o *PublishEventBatchByKeyRequestPEventsInner) SetFields(v map[string]interface{})`

SetFields sets Fields field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



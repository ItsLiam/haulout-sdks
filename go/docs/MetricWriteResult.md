# MetricWriteResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Key** | **string** | Metric identifier, unique within a product. | 
**Revision** | **int64** |  | 
**UpdatedAt** | **time.Time** |  | 

## Methods

### NewMetricWriteResult

`func NewMetricWriteResult(key string, revision int64, updatedAt time.Time, ) *MetricWriteResult`

NewMetricWriteResult instantiates a new MetricWriteResult object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMetricWriteResultWithDefaults

`func NewMetricWriteResultWithDefaults() *MetricWriteResult`

NewMetricWriteResultWithDefaults instantiates a new MetricWriteResult object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKey

`func (o *MetricWriteResult) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *MetricWriteResult) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *MetricWriteResult) SetKey(v string)`

SetKey sets Key field to given value.


### GetRevision

`func (o *MetricWriteResult) GetRevision() int64`

GetRevision returns the Revision field if non-nil, zero value otherwise.

### GetRevisionOk

`func (o *MetricWriteResult) GetRevisionOk() (*int64, bool)`

GetRevisionOk returns a tuple with the Revision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevision

`func (o *MetricWriteResult) SetRevision(v int64)`

SetRevision sets Revision field to given value.


### GetUpdatedAt

`func (o *MetricWriteResult) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *MetricWriteResult) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *MetricWriteResult) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



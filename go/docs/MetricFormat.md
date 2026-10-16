# MetricFormat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Style** | **string** |  | 
**Precision** | Pointer to **int32** |  | [optional] 
**Unit** | Pointer to **string** |  | [optional] 

## Methods

### NewMetricFormat

`func NewMetricFormat(style string, ) *MetricFormat`

NewMetricFormat instantiates a new MetricFormat object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMetricFormatWithDefaults

`func NewMetricFormatWithDefaults() *MetricFormat`

NewMetricFormatWithDefaults instantiates a new MetricFormat object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetStyle

`func (o *MetricFormat) GetStyle() string`

GetStyle returns the Style field if non-nil, zero value otherwise.

### GetStyleOk

`func (o *MetricFormat) GetStyleOk() (*string, bool)`

GetStyleOk returns a tuple with the Style field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStyle

`func (o *MetricFormat) SetStyle(v string)`

SetStyle sets Style field to given value.


### GetPrecision

`func (o *MetricFormat) GetPrecision() int32`

GetPrecision returns the Precision field if non-nil, zero value otherwise.

### GetPrecisionOk

`func (o *MetricFormat) GetPrecisionOk() (*int32, bool)`

GetPrecisionOk returns a tuple with the Precision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrecision

`func (o *MetricFormat) SetPrecision(v int32)`

SetPrecision sets Precision field to given value.

### HasPrecision

`func (o *MetricFormat) HasPrecision() bool`

HasPrecision returns a boolean if a field has been set.

### GetUnit

`func (o *MetricFormat) GetUnit() string`

GetUnit returns the Unit field if non-nil, zero value otherwise.

### GetUnitOk

`func (o *MetricFormat) GetUnitOk() (*string, bool)`

GetUnitOk returns a tuple with the Unit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnit

`func (o *MetricFormat) SetUnit(v string)`

SetUnit sets Unit field to given value.

### HasUnit

`func (o *MetricFormat) HasUnit() bool`

HasUnit returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



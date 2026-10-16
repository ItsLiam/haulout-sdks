# Metric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Key** | **string** | Metric identifier, unique within a product. | 
**Kind** | **string** |  | 
**Label** | Pointer to **string** |  | [optional] 
**Format** | Pointer to [**MetricFormat**](MetricFormat.md) |  | [optional] 
**Value** | **interface{}** | Kind-specific value; see the Metric description. | 

## Methods

### NewMetric

`func NewMetric(key string, kind string, value interface{}, ) *Metric`

NewMetric instantiates a new Metric object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMetricWithDefaults

`func NewMetricWithDefaults() *Metric`

NewMetricWithDefaults instantiates a new Metric object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKey

`func (o *Metric) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *Metric) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *Metric) SetKey(v string)`

SetKey sets Key field to given value.


### GetKind

`func (o *Metric) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *Metric) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *Metric) SetKind(v string)`

SetKind sets Kind field to given value.


### GetLabel

`func (o *Metric) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *Metric) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *Metric) SetLabel(v string)`

SetLabel sets Label field to given value.

### HasLabel

`func (o *Metric) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### GetFormat

`func (o *Metric) GetFormat() MetricFormat`

GetFormat returns the Format field if non-nil, zero value otherwise.

### GetFormatOk

`func (o *Metric) GetFormatOk() (*MetricFormat, bool)`

GetFormatOk returns a tuple with the Format field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormat

`func (o *Metric) SetFormat(v MetricFormat)`

SetFormat sets Format field to given value.

### HasFormat

`func (o *Metric) HasFormat() bool`

HasFormat returns a boolean if a field has been set.

### GetValue

`func (o *Metric) GetValue() interface{}`

GetValue returns the Value field if non-nil, zero value otherwise.

### GetValueOk

`func (o *Metric) GetValueOk() (*interface{}, bool)`

GetValueOk returns a tuple with the Value field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValue

`func (o *Metric) SetValue(v interface{})`

SetValue sets Value field to given value.


### SetValueNil

`func (o *Metric) SetValueNil(b bool)`

 SetValueNil sets the value for Value to be an explicit nil

### UnsetValue
`func (o *Metric) UnsetValue()`

UnsetValue ensures that no value is present for Value, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



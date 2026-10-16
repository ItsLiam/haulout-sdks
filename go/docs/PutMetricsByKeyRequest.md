# PutMetricsByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**PMetrics** | [**[]Metric**](Metric.md) |  | 

## Methods

### NewPutMetricsByKeyRequest

`func NewPutMetricsByKeyRequest(pProductKey string, pMetrics []Metric, ) *PutMetricsByKeyRequest`

NewPutMetricsByKeyRequest instantiates a new PutMetricsByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPutMetricsByKeyRequestWithDefaults

`func NewPutMetricsByKeyRequestWithDefaults() *PutMetricsByKeyRequest`

NewPutMetricsByKeyRequestWithDefaults instantiates a new PutMetricsByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *PutMetricsByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *PutMetricsByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *PutMetricsByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPMetrics

`func (o *PutMetricsByKeyRequest) GetPMetrics() []Metric`

GetPMetrics returns the PMetrics field if non-nil, zero value otherwise.

### GetPMetricsOk

`func (o *PutMetricsByKeyRequest) GetPMetricsOk() (*[]Metric, bool)`

GetPMetricsOk returns a tuple with the PMetrics field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPMetrics

`func (o *PutMetricsByKeyRequest) SetPMetrics(v []Metric)`

SetPMetrics sets PMetrics field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



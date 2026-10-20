# SendNotificationByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**PCategory** | **string** | Notification category identifier, unique within a product. | 
**PTitle** | **string** |  | 
**PBody** | **string** | May be an empty string. | 

## Methods

### NewSendNotificationByKeyRequest

`func NewSendNotificationByKeyRequest(pProductKey string, pCategory string, pTitle string, pBody string, ) *SendNotificationByKeyRequest`

NewSendNotificationByKeyRequest instantiates a new SendNotificationByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSendNotificationByKeyRequestWithDefaults

`func NewSendNotificationByKeyRequestWithDefaults() *SendNotificationByKeyRequest`

NewSendNotificationByKeyRequestWithDefaults instantiates a new SendNotificationByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *SendNotificationByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *SendNotificationByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *SendNotificationByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPCategory

`func (o *SendNotificationByKeyRequest) GetPCategory() string`

GetPCategory returns the PCategory field if non-nil, zero value otherwise.

### GetPCategoryOk

`func (o *SendNotificationByKeyRequest) GetPCategoryOk() (*string, bool)`

GetPCategoryOk returns a tuple with the PCategory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPCategory

`func (o *SendNotificationByKeyRequest) SetPCategory(v string)`

SetPCategory sets PCategory field to given value.


### GetPTitle

`func (o *SendNotificationByKeyRequest) GetPTitle() string`

GetPTitle returns the PTitle field if non-nil, zero value otherwise.

### GetPTitleOk

`func (o *SendNotificationByKeyRequest) GetPTitleOk() (*string, bool)`

GetPTitleOk returns a tuple with the PTitle field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPTitle

`func (o *SendNotificationByKeyRequest) SetPTitle(v string)`

SetPTitle sets PTitle field to given value.


### GetPBody

`func (o *SendNotificationByKeyRequest) GetPBody() string`

GetPBody returns the PBody field if non-nil, zero value otherwise.

### GetPBodyOk

`func (o *SendNotificationByKeyRequest) GetPBodyOk() (*string, bool)`

GetPBodyOk returns a tuple with the PBody field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPBody

`func (o *SendNotificationByKeyRequest) SetPBody(v string)`

SetPBody sets PBody field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# PutNotificationCategoriesByKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**PProductKey** | **string** | The product&#39;s current integration key. Supplied only in p_product_key; rotation invalidates the previous key immediately. | 
**PCategories** | [**[]NotificationCategory**](NotificationCategory.md) |  | 

## Methods

### NewPutNotificationCategoriesByKeyRequest

`func NewPutNotificationCategoriesByKeyRequest(pProductKey string, pCategories []NotificationCategory, ) *PutNotificationCategoriesByKeyRequest`

NewPutNotificationCategoriesByKeyRequest instantiates a new PutNotificationCategoriesByKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPutNotificationCategoriesByKeyRequestWithDefaults

`func NewPutNotificationCategoriesByKeyRequestWithDefaults() *PutNotificationCategoriesByKeyRequest`

NewPutNotificationCategoriesByKeyRequestWithDefaults instantiates a new PutNotificationCategoriesByKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPProductKey

`func (o *PutNotificationCategoriesByKeyRequest) GetPProductKey() string`

GetPProductKey returns the PProductKey field if non-nil, zero value otherwise.

### GetPProductKeyOk

`func (o *PutNotificationCategoriesByKeyRequest) GetPProductKeyOk() (*string, bool)`

GetPProductKeyOk returns a tuple with the PProductKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPProductKey

`func (o *PutNotificationCategoriesByKeyRequest) SetPProductKey(v string)`

SetPProductKey sets PProductKey field to given value.


### GetPCategories

`func (o *PutNotificationCategoriesByKeyRequest) GetPCategories() []NotificationCategory`

GetPCategories returns the PCategories field if non-nil, zero value otherwise.

### GetPCategoriesOk

`func (o *PutNotificationCategoriesByKeyRequest) GetPCategoriesOk() (*[]NotificationCategory, bool)`

GetPCategoriesOk returns a tuple with the PCategories field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPCategories

`func (o *PutNotificationCategoriesByKeyRequest) SetPCategories(v []NotificationCategory)`

SetPCategories sets PCategories field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# NotificationCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Key** | **string** | Notification category identifier, unique within a product. | 
**Label** | **string** |  | 
**Description** | Pointer to **string** |  | [optional] 

## Methods

### NewNotificationCategory

`func NewNotificationCategory(key string, label string, ) *NotificationCategory`

NewNotificationCategory instantiates a new NotificationCategory object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotificationCategoryWithDefaults

`func NewNotificationCategoryWithDefaults() *NotificationCategory`

NewNotificationCategoryWithDefaults instantiates a new NotificationCategory object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKey

`func (o *NotificationCategory) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *NotificationCategory) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *NotificationCategory) SetKey(v string)`

SetKey sets Key field to given value.


### GetLabel

`func (o *NotificationCategory) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *NotificationCategory) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *NotificationCategory) SetLabel(v string)`

SetLabel sets Label field to given value.


### GetDescription

`func (o *NotificationCategory) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *NotificationCategory) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *NotificationCategory) SetDescription(v string)`

SetDescription sets Description field to given value.

### HasDescription

`func (o *NotificationCategory) HasDescription() bool`

HasDescription returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



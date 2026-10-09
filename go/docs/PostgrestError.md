# PostgrestError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | **string** |  | 
**Message** | **string** |  | 
**Details** | Pointer to **NullableString** |  | [optional] 
**Hint** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewPostgrestError

`func NewPostgrestError(code string, message string, ) *PostgrestError`

NewPostgrestError instantiates a new PostgrestError object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPostgrestErrorWithDefaults

`func NewPostgrestErrorWithDefaults() *PostgrestError`

NewPostgrestErrorWithDefaults instantiates a new PostgrestError object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *PostgrestError) GetCode() string`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *PostgrestError) GetCodeOk() (*string, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *PostgrestError) SetCode(v string)`

SetCode sets Code field to given value.


### GetMessage

`func (o *PostgrestError) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PostgrestError) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PostgrestError) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetDetails

`func (o *PostgrestError) GetDetails() string`

GetDetails returns the Details field if non-nil, zero value otherwise.

### GetDetailsOk

`func (o *PostgrestError) GetDetailsOk() (*string, bool)`

GetDetailsOk returns a tuple with the Details field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetails

`func (o *PostgrestError) SetDetails(v string)`

SetDetails sets Details field to given value.

### HasDetails

`func (o *PostgrestError) HasDetails() bool`

HasDetails returns a boolean if a field has been set.

### SetDetailsNil

`func (o *PostgrestError) SetDetailsNil(b bool)`

 SetDetailsNil sets the value for Details to be an explicit nil

### UnsetDetails
`func (o *PostgrestError) UnsetDetails()`

UnsetDetails ensures that no value is present for Details, not even an explicit nil
### GetHint

`func (o *PostgrestError) GetHint() string`

GetHint returns the Hint field if non-nil, zero value otherwise.

### GetHintOk

`func (o *PostgrestError) GetHintOk() (*string, bool)`

GetHintOk returns a tuple with the Hint field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHint

`func (o *PostgrestError) SetHint(v string)`

SetHint sets Hint field to given value.

### HasHint

`func (o *PostgrestError) HasHint() bool`

HasHint returns a boolean if a field has been set.

### SetHintNil

`func (o *PostgrestError) SetHintNil(b bool)`

 SetHintNil sets the value for Hint to be an explicit nil

### UnsetHint
`func (o *PostgrestError) UnsetHint()`

UnsetHint ensures that no value is present for Hint, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



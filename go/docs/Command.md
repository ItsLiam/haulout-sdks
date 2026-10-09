# Command

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Title** | **string** |  | 
**Danger** | **string** |  | 
**Arguments** | [**map[string]Field**](Field.md) |  | 
**TimeoutSeconds** | Pointer to **int32** |  | [optional] 

## Methods

### NewCommand

`func NewCommand(title string, danger string, arguments map[string]Field, ) *Command`

NewCommand instantiates a new Command object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCommandWithDefaults

`func NewCommandWithDefaults() *Command`

NewCommandWithDefaults instantiates a new Command object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTitle

`func (o *Command) GetTitle() string`

GetTitle returns the Title field if non-nil, zero value otherwise.

### GetTitleOk

`func (o *Command) GetTitleOk() (*string, bool)`

GetTitleOk returns a tuple with the Title field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTitle

`func (o *Command) SetTitle(v string)`

SetTitle sets Title field to given value.


### GetDanger

`func (o *Command) GetDanger() string`

GetDanger returns the Danger field if non-nil, zero value otherwise.

### GetDangerOk

`func (o *Command) GetDangerOk() (*string, bool)`

GetDangerOk returns a tuple with the Danger field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDanger

`func (o *Command) SetDanger(v string)`

SetDanger sets Danger field to given value.


### GetArguments

`func (o *Command) GetArguments() map[string]Field`

GetArguments returns the Arguments field if non-nil, zero value otherwise.

### GetArgumentsOk

`func (o *Command) GetArgumentsOk() (*map[string]Field, bool)`

GetArgumentsOk returns a tuple with the Arguments field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetArguments

`func (o *Command) SetArguments(v map[string]Field)`

SetArguments sets Arguments field to given value.


### GetTimeoutSeconds

`func (o *Command) GetTimeoutSeconds() int32`

GetTimeoutSeconds returns the TimeoutSeconds field if non-nil, zero value otherwise.

### GetTimeoutSecondsOk

`func (o *Command) GetTimeoutSecondsOk() (*int32, bool)`

GetTimeoutSecondsOk returns a tuple with the TimeoutSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimeoutSeconds

`func (o *Command) SetTimeoutSeconds(v int32)`

SetTimeoutSeconds sets TimeoutSeconds field to given value.

### HasTimeoutSeconds

`func (o *Command) HasTimeoutSeconds() bool`

HasTimeoutSeconds returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



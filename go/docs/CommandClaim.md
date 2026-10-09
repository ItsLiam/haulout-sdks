# CommandClaim

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CommandId** | **string** |  | 
**Command** | **string** |  | 
**Arguments** | **map[string]interface{}** |  | 
**DefinitionRevision** | **int64** |  | 
**LeaseId** | **string** |  | 
**LeaseExpiresAt** | **time.Time** |  | 

## Methods

### NewCommandClaim

`func NewCommandClaim(commandId string, command string, arguments map[string]interface{}, definitionRevision int64, leaseId string, leaseExpiresAt time.Time, ) *CommandClaim`

NewCommandClaim instantiates a new CommandClaim object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCommandClaimWithDefaults

`func NewCommandClaimWithDefaults() *CommandClaim`

NewCommandClaimWithDefaults instantiates a new CommandClaim object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCommandId

`func (o *CommandClaim) GetCommandId() string`

GetCommandId returns the CommandId field if non-nil, zero value otherwise.

### GetCommandIdOk

`func (o *CommandClaim) GetCommandIdOk() (*string, bool)`

GetCommandIdOk returns a tuple with the CommandId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCommandId

`func (o *CommandClaim) SetCommandId(v string)`

SetCommandId sets CommandId field to given value.


### GetCommand

`func (o *CommandClaim) GetCommand() string`

GetCommand returns the Command field if non-nil, zero value otherwise.

### GetCommandOk

`func (o *CommandClaim) GetCommandOk() (*string, bool)`

GetCommandOk returns a tuple with the Command field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCommand

`func (o *CommandClaim) SetCommand(v string)`

SetCommand sets Command field to given value.


### GetArguments

`func (o *CommandClaim) GetArguments() map[string]interface{}`

GetArguments returns the Arguments field if non-nil, zero value otherwise.

### GetArgumentsOk

`func (o *CommandClaim) GetArgumentsOk() (*map[string]interface{}, bool)`

GetArgumentsOk returns a tuple with the Arguments field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetArguments

`func (o *CommandClaim) SetArguments(v map[string]interface{})`

SetArguments sets Arguments field to given value.


### GetDefinitionRevision

`func (o *CommandClaim) GetDefinitionRevision() int64`

GetDefinitionRevision returns the DefinitionRevision field if non-nil, zero value otherwise.

### GetDefinitionRevisionOk

`func (o *CommandClaim) GetDefinitionRevisionOk() (*int64, bool)`

GetDefinitionRevisionOk returns a tuple with the DefinitionRevision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefinitionRevision

`func (o *CommandClaim) SetDefinitionRevision(v int64)`

SetDefinitionRevision sets DefinitionRevision field to given value.


### GetLeaseId

`func (o *CommandClaim) GetLeaseId() string`

GetLeaseId returns the LeaseId field if non-nil, zero value otherwise.

### GetLeaseIdOk

`func (o *CommandClaim) GetLeaseIdOk() (*string, bool)`

GetLeaseIdOk returns a tuple with the LeaseId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLeaseId

`func (o *CommandClaim) SetLeaseId(v string)`

SetLeaseId sets LeaseId field to given value.


### GetLeaseExpiresAt

`func (o *CommandClaim) GetLeaseExpiresAt() time.Time`

GetLeaseExpiresAt returns the LeaseExpiresAt field if non-nil, zero value otherwise.

### GetLeaseExpiresAtOk

`func (o *CommandClaim) GetLeaseExpiresAtOk() (*time.Time, bool)`

GetLeaseExpiresAtOk returns a tuple with the LeaseExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLeaseExpiresAt

`func (o *CommandClaim) SetLeaseExpiresAt(v time.Time)`

SetLeaseExpiresAt sets LeaseExpiresAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



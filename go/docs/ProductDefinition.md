# ProductDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Type** | **string** |  | 
**Version** | **string** |  | 
**Streams** | [**map[string]Stream**](Stream.md) | Map of declared stream names to HSP stream descriptions. | 
**State** | [**map[string]Field**](Field.md) | Map of declared state paths to field declarations. | 
**Commands** | [**map[string]Command**](Command.md) | Map of declared command names to command declarations. | 
**Dashboards** | **[]map[string]interface{}** | Ordered dashboard descriptors. Their recursively validated HSP 2.0 structures are transported as opaque JSON by this generated DTO. | 

## Methods

### NewProductDefinition

`func NewProductDefinition(type_ string, version string, streams map[string]Stream, state map[string]Field, commands map[string]Command, dashboards []map[string]interface{}, ) *ProductDefinition`

NewProductDefinition instantiates a new ProductDefinition object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProductDefinitionWithDefaults

`func NewProductDefinitionWithDefaults() *ProductDefinition`

NewProductDefinitionWithDefaults instantiates a new ProductDefinition object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetType

`func (o *ProductDefinition) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *ProductDefinition) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *ProductDefinition) SetType(v string)`

SetType sets Type field to given value.


### GetVersion

`func (o *ProductDefinition) GetVersion() string`

GetVersion returns the Version field if non-nil, zero value otherwise.

### GetVersionOk

`func (o *ProductDefinition) GetVersionOk() (*string, bool)`

GetVersionOk returns a tuple with the Version field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVersion

`func (o *ProductDefinition) SetVersion(v string)`

SetVersion sets Version field to given value.


### GetStreams

`func (o *ProductDefinition) GetStreams() map[string]Stream`

GetStreams returns the Streams field if non-nil, zero value otherwise.

### GetStreamsOk

`func (o *ProductDefinition) GetStreamsOk() (*map[string]Stream, bool)`

GetStreamsOk returns a tuple with the Streams field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStreams

`func (o *ProductDefinition) SetStreams(v map[string]Stream)`

SetStreams sets Streams field to given value.


### GetState

`func (o *ProductDefinition) GetState() map[string]Field`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *ProductDefinition) GetStateOk() (*map[string]Field, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *ProductDefinition) SetState(v map[string]Field)`

SetState sets State field to given value.


### GetCommands

`func (o *ProductDefinition) GetCommands() map[string]Command`

GetCommands returns the Commands field if non-nil, zero value otherwise.

### GetCommandsOk

`func (o *ProductDefinition) GetCommandsOk() (*map[string]Command, bool)`

GetCommandsOk returns a tuple with the Commands field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCommands

`func (o *ProductDefinition) SetCommands(v map[string]Command)`

SetCommands sets Commands field to given value.


### GetDashboards

`func (o *ProductDefinition) GetDashboards() []map[string]interface{}`

GetDashboards returns the Dashboards field if non-nil, zero value otherwise.

### GetDashboardsOk

`func (o *ProductDefinition) GetDashboardsOk() (*[]map[string]interface{}, bool)`

GetDashboardsOk returns a tuple with the Dashboards field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDashboards

`func (o *ProductDefinition) SetDashboards(v []map[string]interface{})`

SetDashboards sets Dashboards field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



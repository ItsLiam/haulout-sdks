# \DefaultAPI

All URIs are relative to *https://qszqptdtmkppelqligvb.supabase.co/rest/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ClaimCommandByKey**](DefaultAPI.md#ClaimCommandByKey) | **Post** /rpc/claim_command_by_key | Claim the next eligible command
[**CompleteCommandByKey**](DefaultAPI.md#CompleteCommandByKey) | **Post** /rpc/complete_command_by_key | Complete a leased command
[**PublishEventBatchByKey**](DefaultAPI.md#PublishEventBatchByKey) | **Post** /rpc/publish_event_batch_by_key | Publish an idempotent event batch
[**WriteDefinitionByKey**](DefaultAPI.md#WriteDefinitionByKey) | **Post** /rpc/write_definition_by_key | Compare-and-swap a product definition
[**WriteStateByKey**](DefaultAPI.md#WriteStateByKey) | **Post** /rpc/write_state_by_key | Compare-and-swap product state



## ClaimCommandByKey

> CommandClaim ClaimCommandByKey(ctx).ClaimCommandByKeyRequest(claimCommandByKeyRequest).Execute()

Claim the next eligible command



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	claimCommandByKeyRequest := *openapiclient.NewClaimCommandByKeyRequest("PProductKey_example", "PWorkerId_example", "PClaimId_example") // ClaimCommandByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ClaimCommandByKey(context.Background()).ClaimCommandByKeyRequest(claimCommandByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ClaimCommandByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ClaimCommandByKey`: CommandClaim
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ClaimCommandByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiClaimCommandByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **claimCommandByKeyRequest** | [**ClaimCommandByKeyRequest**](ClaimCommandByKeyRequest.md) |  | 

### Return type

[**CommandClaim**](CommandClaim.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CompleteCommandByKey

> CompletionStatus CompleteCommandByKey(ctx).CompleteCommandByKeyRequest(completeCommandByKeyRequest).Execute()

Complete a leased command



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	completeCommandByKeyRequest := *openapiclient.NewCompleteCommandByKeyRequest("PProductKey_example", "PCommandId_example", "PLeaseId_example", "PStatus_example", interface{}(123)) // CompleteCommandByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CompleteCommandByKey(context.Background()).CompleteCommandByKeyRequest(completeCommandByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CompleteCommandByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CompleteCommandByKey`: CompletionStatus
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CompleteCommandByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCompleteCommandByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **completeCommandByKeyRequest** | [**CompleteCommandByKeyRequest**](CompleteCommandByKeyRequest.md) |  | 

### Return type

[**CompletionStatus**](CompletionStatus.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PublishEventBatchByKey

> PublishEventBatchByKey200Response PublishEventBatchByKey(ctx).PublishEventBatchByKeyRequest(publishEventBatchByKeyRequest).Execute()

Publish an idempotent event batch



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	publishEventBatchByKeyRequest := *openapiclient.NewPublishEventBatchByKeyRequest("PProductKey_example", "PIdempotencyKey_example", int64(123), []openapiclient.PublishEventBatchByKeyRequestPEventsInner{*openapiclient.NewPublishEventBatchByKeyRequestPEventsInner("Id_example", "Source_example", "Time_example", map[string]interface{}{"key": interface{}(123)})}) // PublishEventBatchByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PublishEventBatchByKey(context.Background()).PublishEventBatchByKeyRequest(publishEventBatchByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PublishEventBatchByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PublishEventBatchByKey`: PublishEventBatchByKey200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PublishEventBatchByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPublishEventBatchByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **publishEventBatchByKeyRequest** | [**PublishEventBatchByKeyRequest**](PublishEventBatchByKeyRequest.md) |  | 

### Return type

[**PublishEventBatchByKey200Response**](PublishEventBatchByKey200Response.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## WriteDefinitionByKey

> []WriteDefinitionByKey200ResponseInner WriteDefinitionByKey(ctx).WriteDefinitionByKeyRequest(writeDefinitionByKeyRequest).Execute()

Compare-and-swap a product definition



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	writeDefinitionByKeyRequest := *openapiclient.NewWriteDefinitionByKeyRequest("PProductKey_example", int64(123), *openapiclient.NewProductDefinition("Type_example", "Version_example", map[string]Stream{"key": *openapiclient.NewStream("Kind_example", map[string]Field{"key": *openapiclient.NewField("Type_example")})}, map[string]Field{"key": *openapiclient.NewField("Type_example")}, map[string]Command{"key": *openapiclient.NewCommand("Title_example", "Danger_example", map[string]Field{"key": })}, []map[string]interface{}{map[string]interface{}{"key": interface{}(123)}})) // WriteDefinitionByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.WriteDefinitionByKey(context.Background()).WriteDefinitionByKeyRequest(writeDefinitionByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.WriteDefinitionByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `WriteDefinitionByKey`: []WriteDefinitionByKey200ResponseInner
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.WriteDefinitionByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiWriteDefinitionByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **writeDefinitionByKeyRequest** | [**WriteDefinitionByKeyRequest**](WriteDefinitionByKeyRequest.md) |  | 

### Return type

[**[]WriteDefinitionByKey200ResponseInner**](WriteDefinitionByKey200ResponseInner.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## WriteStateByKey

> []WriteStateByKey200ResponseInner WriteStateByKey(ctx).WriteStateByKeyRequest(writeStateByKeyRequest).Execute()

Compare-and-swap product state



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/ItsLiam/haulout-sdks/go"
)

func main() {
	writeStateByKeyRequest := *openapiclient.NewWriteStateByKeyRequest("PProductKey_example", int64(123), map[string]interface{}{"key": interface{}(123)}) // WriteStateByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.WriteStateByKey(context.Background()).WriteStateByKeyRequest(writeStateByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.WriteStateByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `WriteStateByKey`: []WriteStateByKey200ResponseInner
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.WriteStateByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiWriteStateByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **writeStateByKeyRequest** | [**WriteStateByKeyRequest**](WriteStateByKeyRequest.md) |  | 

### Return type

[**[]WriteStateByKey200ResponseInner**](WriteStateByKey200ResponseInner.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


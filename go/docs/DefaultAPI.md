# \DefaultAPI

All URIs are relative to *https://qszqptdtmkppelqligvb.supabase.co/rest/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteMetricsByKey**](DefaultAPI.md#DeleteMetricsByKey) | **Post** /rpc/delete_metrics_by_key | Delete current metric values
[**PutMetricsByKey**](DefaultAPI.md#PutMetricsByKey) | **Post** /rpc/put_metrics_by_key | Upsert current metric values
[**PutNotificationCategoriesByKey**](DefaultAPI.md#PutNotificationCategoriesByKey) | **Post** /rpc/put_notification_categories_by_key | Replace notification categories
[**SendNotificationByKey**](DefaultAPI.md#SendNotificationByKey) | **Post** /rpc/send_notification_by_key | Send a notification to subscribed devices



## DeleteMetricsByKey

> []DeletedMetric DeleteMetricsByKey(ctx).DeleteMetricsByKeyRequest(deleteMetricsByKeyRequest).Execute()

Delete current metric values



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
	deleteMetricsByKeyRequest := *openapiclient.NewDeleteMetricsByKeyRequest("PProductKey_example", []string{"PKeys_example"}) // DeleteMetricsByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteMetricsByKey(context.Background()).DeleteMetricsByKeyRequest(deleteMetricsByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteMetricsByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteMetricsByKey`: []DeletedMetric
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteMetricsByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiDeleteMetricsByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **deleteMetricsByKeyRequest** | [**DeleteMetricsByKeyRequest**](DeleteMetricsByKeyRequest.md) |  | 

### Return type

[**[]DeletedMetric**](DeletedMetric.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutMetricsByKey

> []MetricWriteResult PutMetricsByKey(ctx).PutMetricsByKeyRequest(putMetricsByKeyRequest).Execute()

Upsert current metric values



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
	putMetricsByKeyRequest := *openapiclient.NewPutMetricsByKeyRequest("PProductKey_example", []openapiclient.Metric{*openapiclient.NewMetric("Key_example", "Kind_example", interface{}(123))}) // PutMetricsByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PutMetricsByKey(context.Background()).PutMetricsByKeyRequest(putMetricsByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PutMetricsByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutMetricsByKey`: []MetricWriteResult
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutMetricsByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPutMetricsByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **putMetricsByKeyRequest** | [**PutMetricsByKeyRequest**](PutMetricsByKeyRequest.md) |  | 

### Return type

[**[]MetricWriteResult**](MetricWriteResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutNotificationCategoriesByKey

> []NotificationCategoryResult PutNotificationCategoriesByKey(ctx).PutNotificationCategoriesByKeyRequest(putNotificationCategoriesByKeyRequest).Execute()

Replace notification categories



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
	putNotificationCategoriesByKeyRequest := *openapiclient.NewPutNotificationCategoriesByKeyRequest("PProductKey_example", []openapiclient.NotificationCategory{*openapiclient.NewNotificationCategory("Key_example", "Label_example")}) // PutNotificationCategoriesByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PutNotificationCategoriesByKey(context.Background()).PutNotificationCategoriesByKeyRequest(putNotificationCategoriesByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PutNotificationCategoriesByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutNotificationCategoriesByKey`: []NotificationCategoryResult
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutNotificationCategoriesByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPutNotificationCategoriesByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **putNotificationCategoriesByKeyRequest** | [**PutNotificationCategoriesByKeyRequest**](PutNotificationCategoriesByKeyRequest.md) |  | 

### Return type

[**[]NotificationCategoryResult**](NotificationCategoryResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## SendNotificationByKey

> []NotificationSendResult SendNotificationByKey(ctx).SendNotificationByKeyRequest(sendNotificationByKeyRequest).Execute()

Send a notification to subscribed devices



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
	sendNotificationByKeyRequest := *openapiclient.NewSendNotificationByKeyRequest("PProductKey_example", "PCategory_example", "PTitle_example", "PBody_example") // SendNotificationByKeyRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SendNotificationByKey(context.Background()).SendNotificationByKeyRequest(sendNotificationByKeyRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SendNotificationByKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SendNotificationByKey`: []NotificationSendResult
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SendNotificationByKey`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiSendNotificationByKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sendNotificationByKeyRequest** | [**SendNotificationByKeyRequest**](SendNotificationByKeyRequest.md) |  | 

### Return type

[**[]NotificationSendResult**](NotificationSendResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


# \DefaultApi

All URIs are relative to *https://qszqptdtmkppelqligvb.supabase.co/rest/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metrics_by_key**](DefaultApi.md#delete_metrics_by_key) | **POST** /rpc/delete_metrics_by_key | Delete current metric values
[**put_metrics_by_key**](DefaultApi.md#put_metrics_by_key) | **POST** /rpc/put_metrics_by_key | Upsert current metric values
[**put_notification_categories_by_key**](DefaultApi.md#put_notification_categories_by_key) | **POST** /rpc/put_notification_categories_by_key | Replace notification categories
[**send_notification_by_key**](DefaultApi.md#send_notification_by_key) | **POST** /rpc/send_notification_by_key | Send a notification to subscribed devices



## delete_metrics_by_key

> Vec<models::DeletedMetric> delete_metrics_by_key(delete_metrics_by_key_request)
Delete current metric values

Deletes the listed metric keys from the product. Unknown keys are ignored, so retries are safe. Returns the keys actually deleted. Accepts 1–256 keys. No argument has a SQL default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_metrics_by_key_request** | [**DeleteMetricsByKeyRequest**](DeleteMetricsByKeyRequest.md) |  | [required] |

### Return type

[**Vec<models::DeletedMetric>**](DeletedMetric.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_metrics_by_key

> Vec<models::MetricWriteResult> put_metrics_by_key(put_metrics_by_key_request)
Upsert current metric values

Upserts 1–100 current metric values for the product. Writes are last-write-wins with no compare-and-swap or idempotency receipt. A put replaces the whole metric; omitted label and format become null. An identical rewrite is a no-op and returns the existing revision. A product may hold at most 256 metrics. Batches are limited to 1 MiB. No argument has a SQL default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_metrics_by_key_request** | [**PutMetricsByKeyRequest**](PutMetricsByKeyRequest.md) |  | [required] |

### Return type

[**Vec<models::MetricWriteResult>**](MetricWriteResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_notification_categories_by_key

> Vec<models::NotificationCategoryResult> put_notification_categories_by_key(put_notification_categories_by_key_request)
Replace notification categories

Replaces the product's whole notification category set with the supplied categories (0–64). Categories omitted from the batch are deleted, and their device subscriptions are removed with them. Keys already present update their label and description in place. An empty array clears every category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_notification_categories_by_key_request** | [**PutNotificationCategoriesByKeyRequest**](PutNotificationCategoriesByKeyRequest.md) |  | [required] |

### Return type

[**Vec<models::NotificationCategoryResult>**](NotificationCategoryResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_notification_by_key

> Vec<models::NotificationSendResult> send_notification_by_key(send_notification_by_key_request)
Send a notification to subscribed devices

Pushes a title and optional body to every device subscribed to the category. Nothing is stored; the payload exists only for dispatch. The recipients count is the number of subscribed devices whose sessions are currently live, so 0 means nobody is subscribed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_notification_by_key_request** | [**SendNotificationByKeyRequest**](SendNotificationByKeyRequest.md) |  | [required] |

### Return type

[**Vec<models::NotificationSendResult>**](NotificationSendResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


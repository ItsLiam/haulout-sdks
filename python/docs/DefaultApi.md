# haulout.DefaultApi

All URIs are relative to *https://qszqptdtmkppelqligvb.supabase.co/rest/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metrics_by_key**](DefaultApi.md#delete_metrics_by_key) | **POST** /rpc/delete_metrics_by_key | Delete current metric values
[**put_metrics_by_key**](DefaultApi.md#put_metrics_by_key) | **POST** /rpc/put_metrics_by_key | Upsert current metric values


# **delete_metrics_by_key**
> List[DeletedMetric] delete_metrics_by_key(delete_metrics_by_key_request)

Delete current metric values

Deletes the listed metric keys from the product. Unknown keys are ignored, so retries are safe. Returns the keys actually deleted. Accepts 1–256 keys. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.delete_metrics_by_key_request import DeleteMetricsByKeyRequest
from haulout.models.deleted_metric import DeletedMetric
from haulout.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://qszqptdtmkppelqligvb.supabase.co/rest/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = haulout.Configuration(
    host = "https://qszqptdtmkppelqligvb.supabase.co/rest/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: HauloutServiceKey
configuration.api_key['HauloutServiceKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['HauloutServiceKey'] = 'Bearer'

# Enter a context with an instance of the API client
with haulout.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = haulout.DefaultApi(api_client)
    delete_metrics_by_key_request = haulout.DeleteMetricsByKeyRequest() # DeleteMetricsByKeyRequest | 

    try:
        # Delete current metric values
        api_response = api_instance.delete_metrics_by_key(delete_metrics_by_key_request)
        print("The response of DefaultApi->delete_metrics_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_metrics_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **delete_metrics_by_key_request** | [**DeleteMetricsByKeyRequest**](DeleteMetricsByKeyRequest.md)|  | 

### Return type

[**List[DeletedMetric]**](DeletedMetric.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Keys deleted by this call, one row per deleted metric. |  -  |
**0** | API error. Code 22023 means invalid or out-of-range keys. Code 28000 means an invalid product key. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_metrics_by_key**
> List[MetricWriteResult] put_metrics_by_key(put_metrics_by_key_request)

Upsert current metric values

Upserts 1–100 current metric values for the product. Writes are last-write-wins with no compare-and-swap or idempotency receipt. A put replaces the whole metric; omitted label and format become null. An identical rewrite is a no-op and returns the existing revision. A product may hold at most 256 metrics. Batches are limited to 1 MiB. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.metric_write_result import MetricWriteResult
from haulout.models.put_metrics_by_key_request import PutMetricsByKeyRequest
from haulout.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://qszqptdtmkppelqligvb.supabase.co/rest/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = haulout.Configuration(
    host = "https://qszqptdtmkppelqligvb.supabase.co/rest/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: HauloutServiceKey
configuration.api_key['HauloutServiceKey'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['HauloutServiceKey'] = 'Bearer'

# Enter a context with an instance of the API client
with haulout.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = haulout.DefaultApi(api_client)
    put_metrics_by_key_request = haulout.PutMetricsByKeyRequest() # PutMetricsByKeyRequest | 

    try:
        # Upsert current metric values
        api_response = api_instance.put_metrics_by_key(put_metrics_by_key_request)
        print("The response of DefaultApi->put_metrics_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_metrics_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **put_metrics_by_key_request** | [**PutMetricsByKeyRequest**](PutMetricsByKeyRequest.md)|  | 

### Return type

[**List[MetricWriteResult]**](MetricWriteResult.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | One row per input metric, in input order. |  -  |
**0** | API error. Code 22023 means an invalid metric, value shape, batch, duplicate key, or product metric limit. Code 28000 means an invalid product key. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


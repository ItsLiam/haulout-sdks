# haulout.DefaultApi

All URIs are relative to *https://qszqptdtmkppelqligvb.supabase.co/rest/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**claim_command_by_key**](DefaultApi.md#claim_command_by_key) | **POST** /rpc/claim_command_by_key | Claim the next eligible command
[**complete_command_by_key**](DefaultApi.md#complete_command_by_key) | **POST** /rpc/complete_command_by_key | Complete a leased command
[**publish_event_batch_by_key**](DefaultApi.md#publish_event_batch_by_key) | **POST** /rpc/publish_event_batch_by_key | Publish an idempotent event batch
[**write_definition_by_key**](DefaultApi.md#write_definition_by_key) | **POST** /rpc/write_definition_by_key | Compare-and-swap a product definition
[**write_state_by_key**](DefaultApi.md#write_state_by_key) | **POST** /rpc/write_state_by_key | Compare-and-swap product state


# **claim_command_by_key**
> CommandClaim claim_command_by_key(claim_command_by_key_request)

Claim the next eligible command

Claims the first eligible accepted command with a 45-second lease. Expired accepted commands become expired; expired running commands become unknown and are not silently rerun. A repeated claim UUID for the same worker returns the same claim; using it for another worker conflicts. HTTP 200 with JSON null means there is no eligible work. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.claim_command_by_key_request import ClaimCommandByKeyRequest
from haulout.models.command_claim import CommandClaim
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
    claim_command_by_key_request = haulout.ClaimCommandByKeyRequest() # ClaimCommandByKeyRequest | 

    try:
        # Claim the next eligible command
        api_response = api_instance.claim_command_by_key(claim_command_by_key_request)
        print("The response of DefaultApi->claim_command_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->claim_command_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **claim_command_by_key_request** | [**ClaimCommandByKeyRequest**](ClaimCommandByKeyRequest.md)|  | 

### Return type

[**CommandClaim**](CommandClaim.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Claimed command object, or JSON null when no work is eligible. |  -  |
**0** | PostgREST error. SQLSTATE 22023 means invalid worker/claim arguments; 28000 means invalid product key; 23505 means claim ID conflict. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **complete_command_by_key**
> CompletionStatus complete_command_by_key(complete_command_by_key_request)

Complete a leased command

Reports succeeded, failed or rejected. Result is arbitrary JSON including scalar, array and null values, and is limited to 131072 bytes. Completion at or after lease expiry records/returns unknown with null result. Repeating the same lease/status/result is idempotent; changed content conflicts. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.complete_command_by_key_request import CompleteCommandByKeyRequest
from haulout.models.completion_status import CompletionStatus
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
    complete_command_by_key_request = haulout.CompleteCommandByKeyRequest() # CompleteCommandByKeyRequest | 

    try:
        # Complete a leased command
        api_response = api_instance.complete_command_by_key(complete_command_by_key_request)
        print("The response of DefaultApi->complete_command_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->complete_command_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **complete_command_by_key_request** | [**CompleteCommandByKeyRequest**](CompleteCommandByKeyRequest.md)|  | 

### Return type

[**CompletionStatus**](CompletionStatus.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Completed status returned directly as a JSON string. |  -  |
**0** | PostgREST error. SQLSTATE 22023 means invalid status/result; 28000 means invalid product key; 42501 means inactive product; 23505 means command result conflict; P0002 means command lease not found. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **publish_event_batch_by_key**
> PublishEventBatchByKey200Response publish_event_batch_by_key(publish_event_batch_by_key_request)

Publish an idempotent event batch

Requires the active definition revision. Accepts 1–100 events; the encoded batch and each event fields object are limited to 131072 bytes. The product-scoped idempotency receipt is valid for 24 hours. Reusing an event ID with identical source, revision, timestamp and fields counts as a duplicate; changed content conflicts. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.publish_event_batch_by_key200_response import PublishEventBatchByKey200Response
from haulout.models.publish_event_batch_by_key_request import PublishEventBatchByKeyRequest
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
    publish_event_batch_by_key_request = haulout.PublishEventBatchByKeyRequest() # PublishEventBatchByKeyRequest | 

    try:
        # Publish an idempotent event batch
        api_response = api_instance.publish_event_batch_by_key(publish_event_batch_by_key_request)
        print("The response of DefaultApi->publish_event_batch_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->publish_event_batch_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **publish_event_batch_by_key_request** | [**PublishEventBatchByKeyRequest**](PublishEventBatchByKeyRequest.md)|  | 

### Return type

[**PublishEventBatchByKey200Response**](PublishEventBatchByKey200Response.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Direct JSON RPC result. |  -  |
**0** | PostgREST error. SQLSTATE 22023 means invalid batch/event/source or event fields; 28000 means invalid product key; PT409 means definition revision conflict; 23505 means idempotency key conflict or event ID reused with different content. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **write_definition_by_key**
> List[WriteDefinitionByKey200ResponseInner] write_definition_by_key(write_definition_by_key_request)

Compare-and-swap a product definition

Replaces the complete HSP ProductDefinition only when the current definition revision equals p_expected_revision. On success the revision increments by one. Definition payloads are limited to 1 MiB. The returned hash is sha256:<lowercase hex>. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.write_definition_by_key200_response_inner import WriteDefinitionByKey200ResponseInner
from haulout.models.write_definition_by_key_request import WriteDefinitionByKeyRequest
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
    write_definition_by_key_request = haulout.WriteDefinitionByKeyRequest() # WriteDefinitionByKeyRequest | 

    try:
        # Compare-and-swap a product definition
        api_response = api_instance.write_definition_by_key(write_definition_by_key_request)
        print("The response of DefaultApi->write_definition_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->write_definition_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **write_definition_by_key_request** | [**WriteDefinitionByKeyRequest**](WriteDefinitionByKeyRequest.md)|  | 

### Return type

[**List[WriteDefinitionByKey200ResponseInner]**](WriteDefinitionByKey200ResponseInner.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | PostgREST table result (one returned row). |  -  |
**0** | PostgREST error. SQLSTATE 22023 means an invalid ProductDefinition; 28000 means invalid product key; PT409 means definition revision conflict. HTTP status translation is performed by PostgREST. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **write_state_by_key**
> List[WriteStateByKey200ResponseInner] write_state_by_key(write_state_by_key_request)

Compare-and-swap product state

Writes the complete state values object after validating declared HSP state fields. An identical existing value is returned without changing its revision, even if the supplied expected revision is stale. Empty initial state has revision zero; its first successful write produces revision one. Payload is limited to 1 MiB. No argument has a SQL default.

### Example

* Api Key Authentication (HauloutServiceKey):

```python
import haulout
from haulout.models.write_state_by_key200_response_inner import WriteStateByKey200ResponseInner
from haulout.models.write_state_by_key_request import WriteStateByKeyRequest
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
    write_state_by_key_request = haulout.WriteStateByKeyRequest() # WriteStateByKeyRequest | 

    try:
        # Compare-and-swap product state
        api_response = api_instance.write_state_by_key(write_state_by_key_request)
        print("The response of DefaultApi->write_state_by_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->write_state_by_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **write_state_by_key_request** | [**WriteStateByKeyRequest**](WriteStateByKeyRequest.md)|  | 

### Return type

[**List[WriteStateByKey200ResponseInner]**](WriteStateByKey200ResponseInner.md)

### Authorization

[HauloutServiceKey](../README.md#HauloutServiceKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | PostgREST table result (one returned row). |  -  |
**0** | PostgREST error. SQLSTATE 22023 means a missing/invalid revision or invalid/undeclared state values; 28000 means invalid product key; PT409 means state revision conflict. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


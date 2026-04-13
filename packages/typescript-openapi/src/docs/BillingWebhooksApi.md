# BillingWebhooksApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**billingWebhookReceive**](#billingwebhookreceive) | **POST** /api/billing/webhook | |

# **billingWebhookReceive**
> billingWebhookReceive(body)


### Example

```typescript
import {
    BillingWebhooksApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingWebhooksApi(configuration);

let body: string; //

const { status, data } = await apiInstance.billingWebhookReceive(
    body
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **body** | **string**|  | |


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


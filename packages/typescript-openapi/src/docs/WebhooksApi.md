# WebhooksApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**webhooksCreate**](#webhookscreate) | **POST** /api/org/{org_id}/form/{form_id}/webhooks | |
|[**webhooksDelete**](#webhooksdelete) | **DELETE** /api/org/{org_id}/form/{form_id}/webhooks/{webhook_id} | |
|[**webhooksList**](#webhookslist) | **GET** /api/org/{org_id}/form/{form_id}/webhooks | |
|[**webhooksListJobs**](#webhookslistjobs) | **GET** /api/org/{org_id}/form/{form_id}/webhooks/{webhook_id}/jobs | |

# **webhooksCreate**
> CreateWebhookResponse webhooksCreate(createWebhookRequest)


### Example

```typescript
import {
    WebhooksApi,
    Configuration,
    CreateWebhookRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new WebhooksApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let createWebhookRequest: CreateWebhookRequest; //

const { status, data } = await apiInstance.webhooksCreate(
    orgId,
    formId,
    createWebhookRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createWebhookRequest** | **CreateWebhookRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|


### Return type

**CreateWebhookResponse**

### Authorization

[org_role_token](../README.md#org_role_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


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

# **webhooksDelete**
> webhooksDelete()


### Example

```typescript
import {
    WebhooksApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WebhooksApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let webhookId: string; // (default to undefined)

const { status, data } = await apiInstance.webhooksDelete(
    orgId,
    formId,
    webhookId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **webhookId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[org_role_token](../README.md#org_role_token)

### HTTP request headers

 - **Content-Type**: Not defined
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

# **webhooksList**
> Array<APIWebhook> webhooksList()


### Example

```typescript
import {
    WebhooksApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WebhooksApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.webhooksList(
    orgId,
    formId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIWebhook>**

### Authorization

[org_role_token](../README.md#org_role_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


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

# **webhooksListJobs**
> Array<APIWebhookJob> webhooksListJobs()


### Example

```typescript
import {
    WebhooksApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WebhooksApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let webhookId: string; // (default to undefined)

const { status, data } = await apiInstance.webhooksListJobs(
    orgId,
    formId,
    webhookId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **webhookId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIWebhookJob>**

### Authorization

[org_role_token](../README.md#org_role_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


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


# FillAccessTokensApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**fillAccessTokensCreate**](#fillaccesstokenscreate) | **POST** /api/org/{org_id}/form/{form_id}/fill_access_tokens | Create a new fill access token.|
|[**fillAccessTokensDelete**](#fillaccesstokensdelete) | **DELETE** /api/org/{org_id}/form/{form_id}/fill_access_tokens/{token_id} | Delete a fill access token.|
|[**fillAccessTokensList**](#fillaccesstokenslist) | **GET** /api/org/{org_id}/form/{form_id}/fill_access_tokens | List all fill access tokens of a form.|

# **fillAccessTokensCreate**
> APIFillToken fillAccessTokensCreate(newTokenRequest)

\\ These tokens are included in fill URLs, which allows end-users to fill in forms. Each form can\\ have multiple fill access tokens, which allows segmenting submissions based on their source.

### Example

```typescript
import {
    FillAccessTokensApi,
    Configuration,
    NewTokenRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FillAccessTokensApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let newTokenRequest: NewTokenRequest; //

const { status, data } = await apiInstance.fillAccessTokensCreate(
    formId,
    orgId,
    newTokenRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newTokenRequest** | **NewTokenRequest**|  | |
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIFillToken**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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

# **fillAccessTokensDelete**
> fillAccessTokensDelete()

\\ The responses created with this token will not be deleted, but the token will be disassociated\\ from them. URLs containing this token will stop working immediately.

### Example

```typescript
import {
    FillAccessTokensApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FillAccessTokensApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let tokenId: string; // (default to undefined)

const { status, data } = await apiInstance.fillAccessTokensDelete(
    formId,
    orgId,
    tokenId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|
| **tokenId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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

# **fillAccessTokensList**
> Array<APIFillToken> fillAccessTokensList()


### Example

```typescript
import {
    FillAccessTokensApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FillAccessTokensApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.fillAccessTokensList(
    formId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIFillToken>**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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


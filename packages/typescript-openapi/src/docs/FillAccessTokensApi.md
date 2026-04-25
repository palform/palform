# FillAccessTokensApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**fillAccessTokensCreate**](#fillaccesstokenscreate) | **POST** /api/org/{org_id}/form/{form_id}/fill_access_tokens | |
|[**fillAccessTokensDelete**](#fillaccesstokensdelete) | **DELETE** /api/org/{org_id}/form/{form_id}/fill_access_tokens/{token_id} | |
|[**fillAccessTokensList**](#fillaccesstokenslist) | **GET** /api/org/{org_id}/form/{form_id}/fill_access_tokens | |

# **fillAccessTokensCreate**
> APIFillToken fillAccessTokensCreate(newTokenRequest)


### Example

```typescript
import {
    FillAccessTokensApi,
    Configuration,
    NewTokenRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FillAccessTokensApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let newTokenRequest: NewTokenRequest; //

const { status, data } = await apiInstance.fillAccessTokensCreate(
    orgId,
    formId,
    newTokenRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newTokenRequest** | **NewTokenRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|


### Return type

**APIFillToken**

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

# **fillAccessTokensDelete**
> fillAccessTokensDelete()


### Example

```typescript
import {
    FillAccessTokensApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FillAccessTokensApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let tokenId: string; // (default to undefined)

const { status, data } = await apiInstance.fillAccessTokensDelete(
    orgId,
    formId,
    tokenId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **tokenId** | [**string**] |  | defaults to undefined|


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

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.fillAccessTokensList(
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

**Array<APIFillToken>**

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


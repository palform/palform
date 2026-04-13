# AdminUsersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**adminUsersUpdate**](#adminusersupdate) | **PATCH** /api/users/me | |

# **adminUsersUpdate**
> adminUsersUpdate(updateAdminUserRequest)


### Example

```typescript
import {
    AdminUsersApi,
    Configuration,
    UpdateAdminUserRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AdminUsersApi(configuration);

let updateAdminUserRequest: UpdateAdminUserRequest; //

const { status, data } = await apiInstance.adminUsersUpdate(
    updateAdminUserRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateAdminUserRequest** | **UpdateAdminUserRequest**|  | |


### Return type

void (empty response body)

### Authorization

[api_auth_token](../README.md#api_auth_token)

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


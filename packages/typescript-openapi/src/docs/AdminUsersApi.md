# AdminUsersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**adminUsersServiceAccountsCreate**](#adminusersserviceaccountscreate) | **POST** /api/org/{org_id}/users/service-accounts | |
|[**adminUsersServiceAccountsDelete**](#adminusersserviceaccountsdelete) | **DELETE** /api/org/{org_id}/users/service-accounts/{admin_user_id} | |
|[**adminUsersUpdate**](#adminusersupdate) | **PATCH** /api/users/me | |

# **adminUsersServiceAccountsCreate**
> APIServiceAccountAdminUser adminUsersServiceAccountsCreate()


### Example

```typescript
import {
    AdminUsersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AdminUsersApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.adminUsersServiceAccountsCreate(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIServiceAccountAdminUser**

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

# **adminUsersServiceAccountsDelete**
> adminUsersServiceAccountsDelete()


### Example

```typescript
import {
    AdminUsersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AdminUsersApi(configuration);

let adminUserId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.adminUsersServiceAccountsDelete(
    adminUserId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminUserId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


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

[personal_token](../README.md#personal_token)

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


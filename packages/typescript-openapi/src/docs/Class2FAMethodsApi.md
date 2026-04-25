# Class2FAMethodsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**userSecondFactorsDelete**](#usersecondfactorsdelete) | **DELETE** /api/auth/tfa/{factor_id} | |
|[**userSecondFactorsEnroll**](#usersecondfactorsenroll) | **POST** /api/auth/tfa/enroll/totp | |
|[**userSecondFactorsEnrollWebauthn**](#usersecondfactorsenrollwebauthn) | **POST** /api/auth/tfa/enroll/webauthn | |
|[**userSecondFactorsList**](#usersecondfactorslist) | **GET** /api/auth/tfa | |
|[**userSecondFactorsStartWebauthn**](#usersecondfactorsstartwebauthn) | **POST** /api/auth/tfa/start/webauthn | |

# **userSecondFactorsDelete**
> userSecondFactorsDelete()


### Example

```typescript
import {
    Class2FAMethodsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new Class2FAMethodsApi(configuration);

let factorId: string; // (default to undefined)

const { status, data } = await apiInstance.userSecondFactorsDelete(
    factorId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **factorId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[api_auth_token](../README.md#api_auth_token)

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

# **userSecondFactorsEnroll**
> string userSecondFactorsEnroll(enrollTOTPRequest)


### Example

```typescript
import {
    Class2FAMethodsApi,
    Configuration,
    EnrollTOTPRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new Class2FAMethodsApi(configuration);

let enrollTOTPRequest: EnrollTOTPRequest; //

const { status, data } = await apiInstance.userSecondFactorsEnroll(
    enrollTOTPRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **enrollTOTPRequest** | **EnrollTOTPRequest**|  | |


### Return type

**string**

### Authorization

[api_auth_token](../README.md#api_auth_token)

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

# **userSecondFactorsEnrollWebauthn**
> string userSecondFactorsEnrollWebauthn(enrollWebauthnRequest)


### Example

```typescript
import {
    Class2FAMethodsApi,
    Configuration,
    EnrollWebauthnRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new Class2FAMethodsApi(configuration);

let enrollWebauthnRequest: EnrollWebauthnRequest; //

const { status, data } = await apiInstance.userSecondFactorsEnrollWebauthn(
    enrollWebauthnRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **enrollWebauthnRequest** | **EnrollWebauthnRequest**|  | |


### Return type

**string**

### Authorization

[api_auth_token](../README.md#api_auth_token)

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

# **userSecondFactorsList**
> Array<APIAdminUserSecondAuthenticationFactor> userSecondFactorsList()


### Example

```typescript
import {
    Class2FAMethodsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new Class2FAMethodsApi(configuration);

const { status, data } = await apiInstance.userSecondFactorsList();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APIAdminUserSecondAuthenticationFactor>**

### Authorization

[api_auth_token](../README.md#api_auth_token)

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

# **userSecondFactorsStartWebauthn**
> StartWebauthnResponse userSecondFactorsStartWebauthn()


### Example

```typescript
import {
    Class2FAMethodsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new Class2FAMethodsApi(configuration);

const { status, data } = await apiInstance.userSecondFactorsStartWebauthn();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**StartWebauthnResponse**

### Authorization

[api_auth_token](../README.md#api_auth_token)

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


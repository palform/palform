# UserKeysApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**keysDelete**](#keysdelete) | **DELETE** /api/org/{org_id}/keys/{key_id} | |
|[**keysGet**](#keysget) | **GET** /api/org/{org_id}/keys/{key_id} | |
|[**keysGetBackup**](#keysgetbackup) | **GET** /api/org/{org_id}/keys/{key_id}/backup | |
|[**keysList**](#keyslist) | **GET** /api/org/{org_id}/keys/my | List user keys|
|[**keysRegister**](#keysregister) | **POST** /api/org/{org_id}/keys/my | Register new public key|
|[**keysRegisterBackup**](#keysregisterbackup) | **PUT** /api/org/{org_id}/keys/{key_id}/backup | |

# **keysDelete**
> keysDelete()


### Example

```typescript
import {
    UserKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)
let keyId: string; // (default to undefined)

const { status, data } = await apiInstance.keysDelete(
    orgId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **keyId** | [**string**] |  | defaults to undefined|


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

# **keysGet**
> APIUserKey keysGet()


### Example

```typescript
import {
    UserKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)
let keyId: string; // (default to undefined)

const { status, data } = await apiInstance.keysGet(
    orgId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **keyId** | [**string**] |  | defaults to undefined|


### Return type

**APIUserKey**

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

# **keysGetBackup**
> string keysGetBackup()


### Example

```typescript
import {
    UserKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)
let keyId: string; // (default to undefined)

const { status, data } = await apiInstance.keysGetBackup(
    orgId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **keyId** | [**string**] |  | defaults to undefined|


### Return type

**string**

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

# **keysList**
> Array<APIUserKey> keysList()

\\ Lists the public keys associated with the currently authenticated user\'s account in PEM-encoded\\ format.

### Example

```typescript
import {
    UserKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.keysList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIUserKey>**

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

# **keysRegister**
> string keysRegister(registerKeyRequest)

\\ Registers a new public to the authenticated user\'s account. The key is stored in DER-encoded\\ binary in the database and can be retrieved using the GET /users/me/key endpoint.

### Example

```typescript
import {
    UserKeysApi,
    Configuration,
    RegisterKeyRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)
let registerKeyRequest: RegisterKeyRequest; //

const { status, data } = await apiInstance.keysRegister(
    orgId,
    registerKeyRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerKeyRequest** | **RegisterKeyRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**string**

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

# **keysRegisterBackup**
> keysRegisterBackup(registerBackupKeyRequest)


### Example

```typescript
import {
    UserKeysApi,
    Configuration,
    RegisterBackupKeyRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let orgId: string; // (default to undefined)
let keyId: string; // (default to undefined)
let registerBackupKeyRequest: RegisterBackupKeyRequest; //

const { status, data } = await apiInstance.keysRegisterBackup(
    orgId,
    keyId,
    registerBackupKeyRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerBackupKeyRequest** | **RegisterBackupKeyRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **keyId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[org_role_token](../README.md#org_role_token)

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


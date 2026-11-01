# UserKeysApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**keysDelete**](#keysdelete) | **DELETE** /api/org/{org_id}/keys/{key_id} | |
|[**keysGet**](#keysget) | **GET** /api/org/{org_id}/keys/{key_id} | |
|[**keysGetBackup**](#keysgetbackup) | **GET** /api/org/{org_id}/keys/{key_id}/backup | |
|[**keysList**](#keyslist) | **GET** /api/org/{org_id}/keys/my | List user keys|
|[**keysRegister**](#keysregister) | **POST** /api/org/{org_id}/keys/my | Register new public key|
|[**keysRegisterBackup**](#keysregisterbackup) | **PUT** /api/org/{org_id}/keys/{key_id}/backup | Register or delete an encrypted backup of an existing public key.|

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

let keyId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.keysDelete(
    keyId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **keyId** | [**string**] |  | defaults to undefined|
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

let keyId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.keysGet(
    keyId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **keyId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIUserKey**

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

let keyId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.keysGetBackup(
    keyId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **keyId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**string**

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

# **keysList**
> Array<APIUserKey> keysList()

\\ Lists the public keys associated with the currently user\'s account within the\\ specified organisation, in PEM-encoded format. Expired keys will also be included in the list.

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

# **keysRegister**
> string keysRegister(registerKeyRequest)

\\ Registers a new public key to the authenticated user\'s account. New responses to forms that are\\ in teams that the user is a member of will start being encrypted with the new key immediately.

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

# **keysRegisterBackup**
> keysRegisterBackup(registerBackupKeyRequest)

\\ The provided secret key should be encrypted with a passphrase. The unencrypted secret key should\\ never under any circumstances be uploaded to the server.\\ \\ Using key backups is completely optional; all other Palform features will work as normal if your\\ key is not backed up. However, it is highly recommended for most users to avoid data loss.

### Example

```typescript
import {
    UserKeysApi,
    Configuration,
    RegisterBackupKeyRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UserKeysApi(configuration);

let keyId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let registerBackupKeyRequest: RegisterBackupKeyRequest; //

const { status, data } = await apiInstance.keysRegisterBackup(
    keyId,
    orgId,
    registerBackupKeyRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerBackupKeyRequest** | **RegisterBackupKeyRequest**|  | |
| **keyId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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


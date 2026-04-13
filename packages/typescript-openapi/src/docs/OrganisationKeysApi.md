# OrganisationKeysApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**orgKeysList**](#orgkeyslist) | **GET** /api/org/{org_id}/keys/all | |
|[**orgKeysTeamFingerprints**](#orgkeysteamfingerprints) | **GET** /api/org/{org_id}/team/{team_id}/keys/all | |

# **orgKeysList**
> Array<APIUserKeyWithIdentity> orgKeysList()


### Example

```typescript
import {
    OrganisationKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationKeysApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.orgKeysList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIUserKeyWithIdentity>**

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

# **orgKeysTeamFingerprints**
> KeysTeamFingerprintsResponse orgKeysTeamFingerprints()


### Example

```typescript
import {
    OrganisationKeysApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationKeysApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.orgKeysTeamFingerprints(
    orgId,
    teamId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|


### Return type

**KeysTeamFingerprintsResponse**

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


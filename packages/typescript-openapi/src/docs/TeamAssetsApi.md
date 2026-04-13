# TeamAssetsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationTeamAssetGet**](#organisationteamassetget) | **GET** /api/org/{org_id}/team/{team_id}/assets/{asset_id} | |
|[**organisationTeamAssetList**](#organisationteamassetlist) | **GET** /api/org/{org_id}/team/{team_id}/assets | |
|[**organisationTeamAssetUpload**](#organisationteamassetupload) | **POST** /api/org/{org_id}/team/{team_id}/assets | |

# **organisationTeamAssetGet**
> string organisationTeamAssetGet()


### Example

```typescript
import {
    TeamAssetsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TeamAssetsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let assetId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamAssetGet(
    orgId,
    teamId,
    assetId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **assetId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamAssetList**
> Array<APITeamAsset> organisationTeamAssetList()


### Example

```typescript
import {
    TeamAssetsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TeamAssetsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamAssetList(
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

**Array<APITeamAsset>**

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

# **organisationTeamAssetUpload**
> APITeamAsset organisationTeamAssetUpload()


### Example

```typescript
import {
    TeamAssetsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TeamAssetsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamAssetUpload(
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

**APITeamAsset**

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


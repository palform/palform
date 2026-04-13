# OrganisationAuthenticationTeamMappingsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationAuthConfigMappingsCreate**](#organisationauthconfigmappingscreate) | **POST** /api/org/{org_id}/mappings | |
|[**organisationAuthConfigMappingsDelete**](#organisationauthconfigmappingsdelete) | **DELETE** /api/org/{org_id}/mappings/{mapping_id} | |
|[**organisationAuthConfigMappingsList**](#organisationauthconfigmappingslist) | **GET** /api/org/{org_id}/mappings | |

# **organisationAuthConfigMappingsCreate**
> string organisationAuthConfigMappingsCreate(aPIOrganisationAuthTeamMappingRequest)


### Example

```typescript
import {
    OrganisationAuthenticationTeamMappingsApi,
    Configuration,
    APIOrganisationAuthTeamMappingRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationAuthenticationTeamMappingsApi(configuration);

let orgId: string; // (default to undefined)
let aPIOrganisationAuthTeamMappingRequest: APIOrganisationAuthTeamMappingRequest; //

const { status, data } = await apiInstance.organisationAuthConfigMappingsCreate(
    orgId,
    aPIOrganisationAuthTeamMappingRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIOrganisationAuthTeamMappingRequest** | **APIOrganisationAuthTeamMappingRequest**|  | |
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

# **organisationAuthConfigMappingsDelete**
> organisationAuthConfigMappingsDelete()


### Example

```typescript
import {
    OrganisationAuthenticationTeamMappingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationAuthenticationTeamMappingsApi(configuration);

let orgId: string; // (default to undefined)
let mappingId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationAuthConfigMappingsDelete(
    orgId,
    mappingId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **mappingId** | [**string**] |  | defaults to undefined|


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

# **organisationAuthConfigMappingsList**
> Array<APIOrganisationAuthTeamMapping> organisationAuthConfigMappingsList()


### Example

```typescript
import {
    OrganisationAuthenticationTeamMappingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationAuthenticationTeamMappingsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationAuthConfigMappingsList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrganisationAuthTeamMapping>**

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


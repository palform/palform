# OrganisationAuthenticationConfigurationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationAuthConfigGet**](#organisationauthconfigget) | **GET** /api/org/{org_id}/auth_config | |
|[**organisationAuthConfigPut**](#organisationauthconfigput) | **PUT** /api/org/{org_id}/auth_config | |

# **organisationAuthConfigGet**
> APIOrganisationAuthConfig organisationAuthConfigGet()


### Example

```typescript
import {
    OrganisationAuthenticationConfigurationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationAuthenticationConfigurationApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationAuthConfigGet(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIOrganisationAuthConfig**

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

# **organisationAuthConfigPut**
> organisationAuthConfigPut()


### Example

```typescript
import {
    OrganisationAuthenticationConfigurationApi,
    Configuration,
    APIOrganisationAuthConfig
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationAuthenticationConfigurationApi(configuration);

let orgId: string; // (default to undefined)
let aPIOrganisationAuthConfig: APIOrganisationAuthConfig; // (optional)

const { status, data } = await apiInstance.organisationAuthConfigPut(
    orgId,
    aPIOrganisationAuthConfig
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIOrganisationAuthConfig** | **APIOrganisationAuthConfig**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


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


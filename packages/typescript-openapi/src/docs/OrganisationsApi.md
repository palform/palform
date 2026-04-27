# OrganisationsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**orgsCreate**](#orgscreate) | **POST** /api/org/orgs | |
|[**orgsCreateSubdomain**](#orgscreatesubdomain) | **POST** /api/org/orgs/{org_id}/subdomain | |
|[**orgsDelete**](#orgsdelete) | **DELETE** /api/org/orgs/{org_id} | |
|[**orgsGet**](#orgsget) | **GET** /api/org/orgs/{org_id} | |
|[**orgsList**](#orgslist) | **GET** /api/org/orgs | List organisation|
|[**orgsRename**](#orgsrename) | **PATCH** /api/org/orgs/{org_id} | |
|[**orgsResolveSubdomain**](#orgsresolvesubdomain) | **GET** /api/org/orgs/for-subdomain | |

# **orgsCreate**
> string orgsCreate(newOrganisationRequest)


### Example

```typescript
import {
    OrganisationsApi,
    Configuration,
    NewOrganisationRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let newOrganisationRequest: NewOrganisationRequest; //

const { status, data } = await apiInstance.orgsCreate(
    newOrganisationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newOrganisationRequest** | **NewOrganisationRequest**|  | |


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

# **orgsCreateSubdomain**
> orgsCreateSubdomain(createSubdomainRequest)


### Example

```typescript
import {
    OrganisationsApi,
    Configuration,
    CreateSubdomainRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let orgId: string; // (default to undefined)
let createSubdomainRequest: CreateSubdomainRequest; //

const { status, data } = await apiInstance.orgsCreateSubdomain(
    orgId,
    createSubdomainRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createSubdomainRequest** | **CreateSubdomainRequest**|  | |
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

# **orgsDelete**
> APIOrganisationManifest orgsDelete(organisationsDeleteRequest)


### Example

```typescript
import {
    OrganisationsApi,
    Configuration,
    OrganisationsDeleteRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let orgId: string; // (default to undefined)
let organisationsDeleteRequest: OrganisationsDeleteRequest; //

const { status, data } = await apiInstance.orgsDelete(
    orgId,
    organisationsDeleteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **organisationsDeleteRequest** | **OrganisationsDeleteRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIOrganisationManifest**

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

# **orgsGet**
> APIOrganisation orgsGet()


### Example

```typescript
import {
    OrganisationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.orgsGet(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIOrganisation**

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

# **orgsList**
> Array<APIOrganisation> orgsList()

\\ List all the organisations that the authenticated user is a member of

### Example

```typescript
import {
    OrganisationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

const { status, data } = await apiInstance.orgsList();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APIOrganisation>**

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

# **orgsRename**
> orgsRename(patchOrgRequest)


### Example

```typescript
import {
    OrganisationsApi,
    Configuration,
    PatchOrgRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let orgId: string; // (default to undefined)
let patchOrgRequest: PatchOrgRequest; //

const { status, data } = await apiInstance.orgsRename(
    orgId,
    patchOrgRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **patchOrgRequest** | **PatchOrgRequest**|  | |
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

# **orgsResolveSubdomain**
> string orgsResolveSubdomain()


### Example

```typescript
import {
    OrganisationsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationsApi(configuration);

let subdomain: string; // (default to undefined)

const { status, data } = await apiInstance.orgsResolveSubdomain(
    subdomain
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **subdomain** | [**string**] |  | defaults to undefined|


### Return type

**string**

### Authorization

No authorization required

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


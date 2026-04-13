# FormBrandingsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**googleFonts**](#googlefonts) | **GET** /api/fonts | |
|[**organisationTeamBrandingAddAccess**](#organisationteambrandingaddaccess) | **POST** /api/org/{org_id}/team/{team_id}/brandings/{branding_id}/access | |
|[**organisationTeamBrandingCreate**](#organisationteambrandingcreate) | **POST** /api/org/{org_id}/team/{team_id}/brandings | |
|[**organisationTeamBrandingDelete**](#organisationteambrandingdelete) | **DELETE** /api/org/{org_id}/team/{team_id}/brandings/{branding_id} | |
|[**organisationTeamBrandingDeleteAccess**](#organisationteambrandingdeleteaccess) | **DELETE** /api/org/{org_id}/team/{team_id}/brandings/{branding_id}/access | |
|[**organisationTeamBrandingList**](#organisationteambrandinglist) | **GET** /api/org/{org_id}/team/{team_id}/brandings | |
|[**organisationTeamBrandingListAccess**](#organisationteambrandinglistaccess) | **GET** /api/org/{org_id}/team/{team_id}/brandings/{branding_id}/access | |
|[**organisationTeamBrandingPut**](#organisationteambrandingput) | **PUT** /api/org/{org_id}/team/{team_id}/brandings/{branding_id} | |

# **googleFonts**
> string googleFonts()


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

const { status, data } = await apiInstance.googleFonts();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**string**

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

# **organisationTeamBrandingAddAccess**
> APIFormBrandingAccess organisationTeamBrandingAddAccess(addAccessRequest)


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration,
    AddAccessRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let brandingId: string; // (default to undefined)
let addAccessRequest: AddAccessRequest; //

const { status, data } = await apiInstance.organisationTeamBrandingAddAccess(
    orgId,
    teamId,
    brandingId,
    addAccessRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **addAccessRequest** | **AddAccessRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **brandingId** | [**string**] |  | defaults to undefined|


### Return type

**APIFormBrandingAccess**

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

# **organisationTeamBrandingCreate**
> string organisationTeamBrandingCreate(aPIFormBrandingRequest)


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration,
    APIFormBrandingRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let aPIFormBrandingRequest: APIFormBrandingRequest; //

const { status, data } = await apiInstance.organisationTeamBrandingCreate(
    orgId,
    teamId,
    aPIFormBrandingRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIFormBrandingRequest** | **APIFormBrandingRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamBrandingDelete**
> organisationTeamBrandingDelete()


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let brandingId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamBrandingDelete(
    orgId,
    teamId,
    brandingId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **brandingId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamBrandingDeleteAccess**
> organisationTeamBrandingDeleteAccess(deleteAccessRequest)


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration,
    DeleteAccessRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let brandingId: string; // (default to undefined)
let deleteAccessRequest: DeleteAccessRequest; //

const { status, data } = await apiInstance.organisationTeamBrandingDeleteAccess(
    orgId,
    teamId,
    brandingId,
    deleteAccessRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **deleteAccessRequest** | **DeleteAccessRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **brandingId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamBrandingList**
> Array<APIFormBranding> organisationTeamBrandingList()


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamBrandingList(
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

**Array<APIFormBranding>**

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

# **organisationTeamBrandingListAccess**
> Array<APIFormBrandingAccess> organisationTeamBrandingListAccess()


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let brandingId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamBrandingListAccess(
    orgId,
    teamId,
    brandingId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **brandingId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIFormBrandingAccess>**

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

# **organisationTeamBrandingPut**
> organisationTeamBrandingPut(aPIFormBrandingRequest)


### Example

```typescript
import {
    FormBrandingsApi,
    Configuration,
    APIFormBrandingRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormBrandingsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let brandingId: string; // (default to undefined)
let aPIFormBrandingRequest: APIFormBrandingRequest; //

const { status, data } = await apiInstance.organisationTeamBrandingPut(
    orgId,
    teamId,
    brandingId,
    aPIFormBrandingRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIFormBrandingRequest** | **APIFormBrandingRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **brandingId** | [**string**] |  | defaults to undefined|


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


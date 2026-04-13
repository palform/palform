# OrganisationInvitesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationInvitesCreate**](#organisationinvitescreate) | **POST** /api/org/{org_id}/invites | |
|[**organisationInvitesDelete**](#organisationinvitesdelete) | **DELETE** /api/org/{org_id}/invites/{invite_id} | |
|[**organisationInvitesList**](#organisationinviteslist) | **GET** /api/org/{org_id}/invites | |
|[**organisationInvitesPreview**](#organisationinvitespreview) | **GET** /api/org/{org_id}/invites/{invite_id}/preview | |

# **organisationInvitesCreate**
> APIOrganisationInvite organisationInvitesCreate(newOrganisationInviteRequest)


### Example

```typescript
import {
    OrganisationInvitesApi,
    Configuration,
    NewOrganisationInviteRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationInvitesApi(configuration);

let orgId: string; // (default to undefined)
let newOrganisationInviteRequest: NewOrganisationInviteRequest; //

const { status, data } = await apiInstance.organisationInvitesCreate(
    orgId,
    newOrganisationInviteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newOrganisationInviteRequest** | **NewOrganisationInviteRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIOrganisationInvite**

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

# **organisationInvitesDelete**
> organisationInvitesDelete()


### Example

```typescript
import {
    OrganisationInvitesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationInvitesApi(configuration);

let orgId: string; // (default to undefined)
let inviteId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationInvitesDelete(
    orgId,
    inviteId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **inviteId** | [**string**] |  | defaults to undefined|


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

# **organisationInvitesList**
> Array<APIOrganisationInvite> organisationInvitesList()


### Example

```typescript
import {
    OrganisationInvitesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationInvitesApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationInvitesList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrganisationInvite>**

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

# **organisationInvitesPreview**
> APIOrganisationInvitePreview organisationInvitesPreview()


### Example

```typescript
import {
    OrganisationInvitesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationInvitesApi(configuration);

let orgId: string; // (default to undefined)
let inviteId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationInvitesPreview(
    orgId,
    inviteId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **inviteId** | [**string**] |  | defaults to undefined|


### Return type

**APIOrganisationInvitePreview**

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


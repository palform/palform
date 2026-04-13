# OrganisationTeamMembersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationTeamMembersAdd**](#organisationteammembersadd) | **POST** /api/org/{org_id}/team/{team_id}/members | |
|[**organisationTeamMembersDelete**](#organisationteammembersdelete) | **DELETE** /api/org/{org_id}/team/{team_id}/members/{member_user_id} | |
|[**organisationTeamMembersList**](#organisationteammemberslist) | **GET** /api/org/{org_id}/team/{team_id}/members | |
|[**organisationTeamMembersPatch**](#organisationteammemberspatch) | **PATCH** /api/org/{org_id}/team/{team_id}/members/{member_user_id} | |

# **organisationTeamMembersAdd**
> organisationTeamMembersAdd(addTeamMemberRequest)


### Example

```typescript
import {
    OrganisationTeamMembersApi,
    Configuration,
    AddTeamMemberRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamMembersApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let addTeamMemberRequest: AddTeamMemberRequest; //

const { status, data } = await apiInstance.organisationTeamMembersAdd(
    orgId,
    teamId,
    addTeamMemberRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **addTeamMemberRequest** | **AddTeamMemberRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamMembersDelete**
> organisationTeamMembersDelete()


### Example

```typescript
import {
    OrganisationTeamMembersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamMembersApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let memberUserId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamMembersDelete(
    orgId,
    teamId,
    memberUserId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **memberUserId** | [**string**] |  | defaults to undefined|


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

# **organisationTeamMembersList**
> Array<APIOrganisationTeamMember> organisationTeamMembersList()


### Example

```typescript
import {
    OrganisationTeamMembersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamMembersApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamMembersList(
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

**Array<APIOrganisationTeamMember>**

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

# **organisationTeamMembersPatch**
> organisationTeamMembersPatch(organisationTeamMembersPatchRequest)


### Example

```typescript
import {
    OrganisationTeamMembersApi,
    Configuration,
    OrganisationTeamMembersPatchRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamMembersApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)
let memberUserId: string; // (default to undefined)
let organisationTeamMembersPatchRequest: OrganisationTeamMembersPatchRequest; //

const { status, data } = await apiInstance.organisationTeamMembersPatch(
    orgId,
    teamId,
    memberUserId,
    organisationTeamMembersPatchRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **organisationTeamMembersPatchRequest** | **OrganisationTeamMembersPatchRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **teamId** | [**string**] |  | defaults to undefined|
| **memberUserId** | [**string**] |  | defaults to undefined|


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


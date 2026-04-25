# OrganisationTeamsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationTeamsCreate**](#organisationteamscreate) | **POST** /api/org/{org_id}/team/teams | |
|[**organisationTeamsDelete**](#organisationteamsdelete) | **DELETE** /api/org/{org_id}/team/teams/{team_id} | |
|[**organisationTeamsGet**](#organisationteamsget) | **GET** /api/org/{org_id}/team/teams/{team_id} | |
|[**organisationTeamsList**](#organisationteamslist) | **GET** /api/org/{org_id}/team/teams | |
|[**organisationTeamsListMy**](#organisationteamslistmy) | **GET** /api/org/{org_id}/team/teams/my | |

# **organisationTeamsCreate**
> string organisationTeamsCreate(createTeamRequest)


### Example

```typescript
import {
    OrganisationTeamsApi,
    Configuration,
    CreateTeamRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamsApi(configuration);

let orgId: string; // (default to undefined)
let createTeamRequest: CreateTeamRequest; //

const { status, data } = await apiInstance.organisationTeamsCreate(
    orgId,
    createTeamRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createTeamRequest** | **CreateTeamRequest**|  | |
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

# **organisationTeamsDelete**
> organisationTeamsDelete()


### Example

```typescript
import {
    OrganisationTeamsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamsDelete(
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

# **organisationTeamsGet**
> APIOrganisationTeam organisationTeamsGet()


### Example

```typescript
import {
    OrganisationTeamsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamsApi(configuration);

let orgId: string; // (default to undefined)
let teamId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamsGet(
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

**APIOrganisationTeam**

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

# **organisationTeamsList**
> Array<APIOrganisationTeam> organisationTeamsList()


### Example

```typescript
import {
    OrganisationTeamsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamsList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrganisationTeam>**

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

# **organisationTeamsListMy**
> Array<APIOrganisationTeamMembership> organisationTeamsListMy()


### Example

```typescript
import {
    OrganisationTeamsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationTeamsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationTeamsListMy(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrganisationTeamMembership>**

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


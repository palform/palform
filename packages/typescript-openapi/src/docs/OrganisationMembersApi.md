# OrganisationMembersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationMembersAmIAdmin**](#organisationmembersamiadmin) | **GET** /api/org/{org_id}/members/am-i-admin | |
|[**organisationMembersDelete**](#organisationmembersdelete) | **DELETE** /api/org/{org_id}/members/{user_id} | |
|[**organisationMembersJoin**](#organisationmembersjoin) | **POST** /api/org/{org_id}/members | |
|[**organisationMembersList**](#organisationmemberslist) | **GET** /api/org/{org_id}/members | |
|[**organisationMembersPatch**](#organisationmemberspatch) | **PATCH** /api/org/{org_id}/members/{user_id} | |

# **organisationMembersAmIAdmin**
> boolean organisationMembersAmIAdmin()


### Example

```typescript
import {
    OrganisationMembersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationMembersApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationMembersAmIAdmin(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**boolean**

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

# **organisationMembersDelete**
> organisationMembersDelete()


### Example

```typescript
import {
    OrganisationMembersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationMembersApi(configuration);

let orgId: string; // (default to undefined)
let userId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationMembersDelete(
    orgId,
    userId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **userId** | [**string**] |  | defaults to undefined|


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

# **organisationMembersJoin**
> organisationMembersJoin(joinOrganisationRequest)


### Example

```typescript
import {
    OrganisationMembersApi,
    Configuration,
    JoinOrganisationRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationMembersApi(configuration);

let orgId: string; // (default to undefined)
let joinOrganisationRequest: JoinOrganisationRequest; //

const { status, data } = await apiInstance.organisationMembersJoin(
    orgId,
    joinOrganisationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **joinOrganisationRequest** | **JoinOrganisationRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[api_auth_token](../README.md#api_auth_token)

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

# **organisationMembersList**
> Array<APIOrgMember> organisationMembersList()


### Example

```typescript
import {
    OrganisationMembersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationMembersApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationMembersList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrgMember>**

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

# **organisationMembersPatch**
> organisationMembersPatch(patchOrgMemberRequest)


### Example

```typescript
import {
    OrganisationMembersApi,
    Configuration,
    PatchOrgMemberRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationMembersApi(configuration);

let orgId: string; // (default to undefined)
let userId: string; // (default to undefined)
let patchOrgMemberRequest: PatchOrgMemberRequest; //

const { status, data } = await apiInstance.organisationMembersPatch(
    orgId,
    userId,
    patchOrgMemberRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **patchOrgMemberRequest** | **PatchOrgMemberRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **userId** | [**string**] |  | defaults to undefined|


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


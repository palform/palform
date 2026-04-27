# OrganisationDeletionRequestsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**organisationDeletionRequestsCancel**](#organisationdeletionrequestscancel) | **DELETE** /api/org/{org_id}/deletion_requests/{request_id} | |
|[**organisationDeletionRequestsList**](#organisationdeletionrequestslist) | **GET** /api/org/{org_id}/deletion_requests | |
|[**organisationDeletionRequestsSkip**](#organisationdeletionrequestsskip) | **POST** /api/org/{org_id}/deletion_requests/{request_id}/skip | |

# **organisationDeletionRequestsCancel**
> organisationDeletionRequestsCancel()


### Example

```typescript
import {
    OrganisationDeletionRequestsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationDeletionRequestsApi(configuration);

let orgId: string; // (default to undefined)
let requestId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationDeletionRequestsCancel(
    orgId,
    requestId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **requestId** | [**string**] |  | defaults to undefined|


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

# **organisationDeletionRequestsList**
> Array<APIOrganisationDeletionRequest> organisationDeletionRequestsList()


### Example

```typescript
import {
    OrganisationDeletionRequestsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationDeletionRequestsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationDeletionRequestsList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIOrganisationDeletionRequest>**

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

# **organisationDeletionRequestsSkip**
> organisationDeletionRequestsSkip()


### Example

```typescript
import {
    OrganisationDeletionRequestsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new OrganisationDeletionRequestsApi(configuration);

let orgId: string; // (default to undefined)
let requestId: string; // (default to undefined)

const { status, data } = await apiInstance.organisationDeletionRequestsSkip(
    orgId,
    requestId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **requestId** | [**string**] |  | defaults to undefined|


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


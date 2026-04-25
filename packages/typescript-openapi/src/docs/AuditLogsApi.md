# AuditLogsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**auditList**](#auditlist) | **GET** /api/org/{org_id}/audit | |

# **auditList**
> Array<APIAuditLogEntry> auditList()


### Example

```typescript
import {
    AuditLogsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuditLogsApi(configuration);

let orgId: string; // (default to undefined)
let from: string; // (optional) (default to undefined)
let resource: AuditLogTargetResourceEnum; // (optional) (default to undefined)
let to: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.auditList(
    orgId,
    from,
    resource,
    to
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **from** | [**string**] |  | (optional) defaults to undefined|
| **resource** | **AuditLogTargetResourceEnum** |  | (optional) defaults to undefined|
| **to** | [**string**] |  | (optional) defaults to undefined|


### Return type

**Array<APIAuditLogEntry>**

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


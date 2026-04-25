# SubmissionsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**submissionsCrypto**](#submissionscrypto) | **GET** /api/org/{org_id}/form/{form_id}/submissions/{submission_id}/crypto | |
|[**submissionsDelete**](#submissionsdelete) | **DELETE** /api/org/{org_id}/form/{form_id}/submissions/{submission_id} | |
|[**submissionsList**](#submissionslist) | **GET** /api/org/{org_id}/form/{form_id}/submissions | |
|[**submissionsNumSince**](#submissionsnumsince) | **POST** /api/org/{org_id}/submissions | |

# **submissionsCrypto**
> SubmissionCryptoDetailsResponse submissionsCrypto()


### Example

```typescript
import {
    SubmissionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let submissionId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionsCrypto(
    orgId,
    formId,
    submissionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **submissionId** | [**string**] |  | defaults to undefined|


### Return type

**SubmissionCryptoDetailsResponse**

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

# **submissionsDelete**
> submissionsDelete()


### Example

```typescript
import {
    SubmissionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let submissionId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionsDelete(
    orgId,
    formId,
    submissionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **submissionId** | [**string**] |  | defaults to undefined|


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

# **submissionsList**
> APISubmissionStream submissionsList()


### Example

```typescript
import {
    SubmissionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let since: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.submissionsList(
    orgId,
    formId,
    since
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **since** | **string** |  | (optional) defaults to undefined|


### Return type

**APISubmissionStream**

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

# **submissionsNumSince**
> Array<APISubmissionCountPerForm> submissionsNumSince(submissionCountSinceRequest)


### Example

```typescript
import {
    SubmissionsApi,
    Configuration,
    SubmissionCountSinceRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let orgId: string; // (default to undefined)
let submissionCountSinceRequest: SubmissionCountSinceRequest; //

const { status, data } = await apiInstance.submissionsNumSince(
    orgId,
    submissionCountSinceRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **submissionCountSinceRequest** | **SubmissionCountSinceRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APISubmissionCountPerForm>**

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


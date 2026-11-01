# SubmissionsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**submissionsCrypto**](#submissionscrypto) | **GET** /api/org/{org_id}/form/{form_id}/submissions/{submission_id}/crypto | List all the keys that are able to decrypt a given form response.|
|[**submissionsDelete**](#submissionsdelete) | **DELETE** /api/org/{org_id}/form/{form_id}/submissions/{submission_id} | Delete a form response.|
|[**submissionsList**](#submissionslist) | **GET** /api/org/{org_id}/form/{form_id}/submissions | List all changes to responses to a form.|
|[**submissionsNumSince**](#submissionsnumsince) | **POST** /api/org/{org_id}/submissions | List the number of new responses to all forms in an organisation since a given date.|

# **submissionsCrypto**
> SubmissionCryptoDetailsResponse submissionsCrypto()

\\ This extracts the recipient keys of the submission\'s PGP message and tries to match each one to\\ a key of a known Palform user. If it succeeds, it returns the full details of the matched key.\\ Otherwise, it just returns the key\'s fingerprint.

### Example

```typescript
import {
    SubmissionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let submissionId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionsCrypto(
    formId,
    orgId,
    submissionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|
| **submissionId** | [**string**] |  | defaults to undefined|


### Return type

**SubmissionCryptoDetailsResponse**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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

\\ The response will be fully deleted, but its ID and deletion time will be retained to allow\\ tracking the fact that a submission was deleted.

### Example

```typescript
import {
    SubmissionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionsApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let submissionId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionsDelete(
    formId,
    orgId,
    submissionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|
| **submissionId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let since: string; //If specified, only list new changes made since this date. (optional) (default to undefined)

const { status, data } = await apiInstance.submissionsList(
    formId,
    orgId,
    since
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|
| **since** | **string** | If specified, only list new changes made since this date. | (optional) defaults to undefined|


### Return type

**APISubmissionStream**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

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


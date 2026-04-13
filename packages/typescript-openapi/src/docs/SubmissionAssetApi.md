# SubmissionAssetApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**submissionAssetsGetLink**](#submissionassetsgetlink) | **GET** /api/org/{org_id}/form/{form_id}/submissions/assets/{file_id} | |
|[**submissionAssetsUpload**](#submissionassetsupload) | **POST** /api/fill/form/{form_id}/org/{org_id}/submission_assets | |

# **submissionAssetsGetLink**
> number submissionAssetsGetLink()


### Example

```typescript
import {
    SubmissionAssetApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionAssetApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let fileId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionAssetsGetLink(
    orgId,
    formId,
    fileId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **fileId** | [**string**] |  | defaults to undefined|


### Return type

**number**

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

# **submissionAssetsUpload**
> SubmissionAssetsUploadResponse submissionAssetsUpload()


### Example

```typescript
import {
    SubmissionAssetApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubmissionAssetApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.submissionAssetsUpload(
    formId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**SubmissionAssetsUploadResponse**

### Authorization

[api_fill_access_token](../README.md#api_fill_access_token)

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


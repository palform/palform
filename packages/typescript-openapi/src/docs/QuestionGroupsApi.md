# QuestionGroupsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**questionGroupsList**](#questiongroupslist) | **GET** /api/org/{org_id}/form/{form_id}/groups | |

# **questionGroupsList**
> Array<APIQuestionGroup> questionGroupsList()


### Example

```typescript
import {
    QuestionGroupsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new QuestionGroupsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.questionGroupsList(
    orgId,
    formId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIQuestionGroup>**

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


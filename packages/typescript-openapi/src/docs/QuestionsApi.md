# QuestionsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**questionsGet**](#questionsget) | **GET** /api/org/{org_id}/form/{form_id}/content/groups/{question_group_id}/questions/{question_id} | |
|[**questionsList**](#questionslist) | **GET** /api/org/{org_id}/form/{form_id}/content/groups/all/questions | |
|[**questionsSave**](#questionssave) | **POST** /api/org/{org_id}/form/{form_id}/content/save | |

# **questionsGet**
> APIQuestion questionsGet()


### Example

```typescript
import {
    QuestionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new QuestionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let questionGroupId: string; // (default to undefined)
let questionId: string; // (default to undefined)

const { status, data } = await apiInstance.questionsGet(
    orgId,
    formId,
    questionGroupId,
    questionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|
| **questionGroupId** | [**string**] |  | defaults to undefined|
| **questionId** | [**string**] |  | defaults to undefined|


### Return type

**APIQuestion**

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

# **questionsList**
> Array<APIQuestion> questionsList()


### Example

```typescript
import {
    QuestionsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new QuestionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.questionsList(
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

**Array<APIQuestion>**

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

# **questionsSave**
> questionsSave(aPISaveQuestionsRequest)


### Example

```typescript
import {
    QuestionsApi,
    Configuration,
    APISaveQuestionsRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new QuestionsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let aPISaveQuestionsRequest: APISaveQuestionsRequest; //

const { status, data } = await apiInstance.questionsSave(
    orgId,
    formId,
    aPISaveQuestionsRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPISaveQuestionsRequest** | **APISaveQuestionsRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **formId** | [**string**] |  | defaults to undefined|


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


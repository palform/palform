# FeedbackApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**feedbackCreate**](#feedbackcreate) | **POST** /api/feedback | |

# **feedbackCreate**
> feedbackCreate(aPIFeedbackItem)


### Example

```typescript
import {
    FeedbackApi,
    Configuration,
    APIFeedbackItem
} from './api';

const configuration = new Configuration();
const apiInstance = new FeedbackApi(configuration);

let aPIFeedbackItem: APIFeedbackItem; //

const { status, data } = await apiInstance.feedbackCreate(
    aPIFeedbackItem
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIFeedbackItem** | **APIFeedbackItem**|  | |


### Return type

void (empty response body)

### Authorization

No authorization required

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


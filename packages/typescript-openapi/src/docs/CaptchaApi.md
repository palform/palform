# CaptchaApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**captchaCreate**](#captchacreate) | **POST** /api/captcha | |

# **captchaCreate**
> APICaptchaChallenge captchaCreate()


### Example

```typescript
import {
    CaptchaApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CaptchaApi(configuration);

const { status, data } = await apiInstance.captchaCreate();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**APICaptchaChallenge**

### Authorization

No authorization required

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


# FormTemplatesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**formTemplatesCategoriesGet**](#formtemplatescategoriesget) | **GET** /api/templates/categories/{category_id} | |
|[**formTemplatesCategoriesList**](#formtemplatescategorieslist) | **GET** /api/templates/categories | |
|[**formTemplatesClone**](#formtemplatesclone) | **POST** /api/org/{org_id}/templates/{template_id}/clone | |
|[**formTemplatesGet**](#formtemplatesget) | **GET** /api/templates/{template_id} | |
|[**formTemplatesList**](#formtemplateslist) | **GET** /api/templates/categories/{category_id}/all | |
|[**formTemplatesListTop**](#formtemplateslisttop) | **GET** /api/templates/top | |
|[**formTemplatesReportView**](#formtemplatesreportview) | **POST** /api/templates/{template_id}/views | |

# **formTemplatesCategoriesGet**
> APIFormTemplateCategory formTemplatesCategoriesGet()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

let categoryId: string; // (default to undefined)

const { status, data } = await apiInstance.formTemplatesCategoriesGet(
    categoryId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **categoryId** | [**string**] |  | defaults to undefined|


### Return type

**APIFormTemplateCategory**

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

# **formTemplatesCategoriesList**
> Array<APIFormTemplateCategory> formTemplatesCategoriesList()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

const { status, data } = await apiInstance.formTemplatesCategoriesList();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APIFormTemplateCategory>**

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

# **formTemplatesClone**
> APIForm formTemplatesClone(cloneFormTemplateRequest)


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration,
    CloneFormTemplateRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

let orgId: string; // (default to undefined)
let templateId: string; // (default to undefined)
let cloneFormTemplateRequest: CloneFormTemplateRequest; //

const { status, data } = await apiInstance.formTemplatesClone(
    orgId,
    templateId,
    cloneFormTemplateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **cloneFormTemplateRequest** | **CloneFormTemplateRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **templateId** | [**string**] |  | defaults to undefined|


### Return type

**APIForm**

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

# **formTemplatesGet**
> APIFormTemplate formTemplatesGet()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

let templateId: string; // (default to undefined)

const { status, data } = await apiInstance.formTemplatesGet(
    templateId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **templateId** | [**string**] |  | defaults to undefined|


### Return type

**APIFormTemplate**

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

# **formTemplatesList**
> Array<APIFormTemplate> formTemplatesList()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

let categoryId: string; // (default to undefined)

const { status, data } = await apiInstance.formTemplatesList(
    categoryId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **categoryId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIFormTemplate>**

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

# **formTemplatesListTop**
> Array<APIFormTemplate> formTemplatesListTop()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

const { status, data } = await apiInstance.formTemplatesListTop();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APIFormTemplate>**

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

# **formTemplatesReportView**
> formTemplatesReportView()


### Example

```typescript
import {
    FormTemplatesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormTemplatesApi(configuration);

let templateId: string; // (default to undefined)

const { status, data } = await apiInstance.formTemplatesReportView(
    templateId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **templateId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

No authorization required

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


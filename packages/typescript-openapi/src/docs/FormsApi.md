# FormsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**formsCreate**](#formscreate) | **POST** /api/org/{org_id}/form/forms | |
|[**formsDelete**](#formsdelete) | **DELETE** /api/org/{org_id}/form/forms/{form_id} | |
|[**formsExchangeShortLink**](#formsexchangeshortlink) | **GET** /api/fill/form/forms/short_link/{subdomain}/{short_link} | |
|[**formsFill**](#formsfill) | **POST** /api/fill/form/forms/{form_id}/org/{org_id} | |
|[**formsGet**](#formsget) | **GET** /api/org/{org_id}/form/forms/{form_id} | |
|[**formsKeys**](#formskeys) | **GET** /api/fill/form/forms/{form_id}/org/{org_id}/keys | |
|[**formsList**](#formslist) | **GET** /api/org/{org_id}/form/forms | |
|[**formsRelocate**](#formsrelocate) | **PATCH** /api/org/{org_id}/form/forms/{form_id}/location | |
|[**formsSetAutoDelete**](#formssetautodelete) | **PUT** /api/org/{org_id}/form/forms/{form_id}/auto-delete | |
|[**formsUpdate**](#formsupdate) | **PUT** /api/org/{org_id}/form/forms/{form_id} | |
|[**formsView**](#formsview) | **GET** /api/fill/form/forms/{form_id}/org/{org_id} | |

# **formsCreate**
> APIForm formsCreate(createFormRequest)


### Example

```typescript
import {
    FormsApi,
    Configuration,
    CreateFormRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let createFormRequest: CreateFormRequest; //

const { status, data } = await apiInstance.formsCreate(
    orgId,
    createFormRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createFormRequest** | **CreateFormRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


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

# **formsDelete**
> formsDelete()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.formsDelete(
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

# **formsExchangeShortLink**
> APIExchangedShortLink formsExchangeShortLink()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let subdomain: string; // (default to undefined)
let shortLink: string; // (default to undefined)

const { status, data } = await apiInstance.formsExchangeShortLink(
    subdomain,
    shortLink
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **subdomain** | [**string**] |  | defaults to undefined|
| **shortLink** | [**string**] |  | defaults to undefined|


### Return type

**APIExchangedShortLink**

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

# **formsFill**
> formsFill(formsFillRequest)


### Example

```typescript
import {
    FormsApi,
    Configuration,
    FormsFillRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)
let formsFillRequest: FormsFillRequest; //

const { status, data } = await apiInstance.formsFill(
    formId,
    orgId,
    formsFillRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formsFillRequest** | **FormsFillRequest**|  | |
| **formId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[verified_captcha](../README.md#verified_captcha), [api_fill_access_token](../README.md#api_fill_access_token)

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

# **formsGet**
> APIForm formsGet()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)

const { status, data } = await apiInstance.formsGet(
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

**APIForm**

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

# **formsKeys**
> FormsKeysResponse formsKeys()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.formsKeys(
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

**FormsKeysResponse**

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

# **formsList**
> Array<APIForm> formsList()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.formsList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIForm>**

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

# **formsRelocate**
> formsRelocate(formsRelocateRequest)


### Example

```typescript
import {
    FormsApi,
    Configuration,
    FormsRelocateRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let formsRelocateRequest: FormsRelocateRequest; //

const { status, data } = await apiInstance.formsRelocate(
    orgId,
    formId,
    formsRelocateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **formsRelocateRequest** | **FormsRelocateRequest**|  | |
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

# **formsSetAutoDelete**
> formsSetAutoDelete(setSubmissionAutoDeleteRequest)


### Example

```typescript
import {
    FormsApi,
    Configuration,
    SetSubmissionAutoDeleteRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let setSubmissionAutoDeleteRequest: SetSubmissionAutoDeleteRequest; //

const { status, data } = await apiInstance.formsSetAutoDelete(
    orgId,
    formId,
    setSubmissionAutoDeleteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **setSubmissionAutoDeleteRequest** | **SetSubmissionAutoDeleteRequest**|  | |
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

# **formsUpdate**
> formsUpdate(updateFormRequest)


### Example

```typescript
import {
    FormsApi,
    Configuration,
    UpdateFormRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let orgId: string; // (default to undefined)
let formId: string; // (default to undefined)
let updateFormRequest: UpdateFormRequest; //

const { status, data } = await apiInstance.formsUpdate(
    orgId,
    formId,
    updateFormRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateFormRequest** | **UpdateFormRequest**|  | |
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

# **formsView**
> APIFormWithQuestions formsView()


### Example

```typescript
import {
    FormsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new FormsApi(configuration);

let formId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.formsView(
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

**APIFormWithQuestions**

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


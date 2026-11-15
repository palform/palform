# LegalAgreementsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**legalAgreementsCreate**](#legalagreementscreate) | **POST** /api/org/{org_id}/legal_agreements/variables/{agreement_type} | |
|[**legalAgreementsDelete**](#legalagreementsdelete) | **DELETE** /api/org/{org_id}/legal_agreements/{agreement_id} | |
|[**legalAgreementsList**](#legalagreementslist) | **GET** /api/org/{org_id}/legal_agreements | |
|[**legalAgreementsListAgreementTypes**](#legalagreementslistagreementtypes) | **GET** /api/org/{org_id}/legal_agreements/types | |
|[**legalAgreementsListVariables**](#legalagreementslistvariables) | **GET** /api/org/{org_id}/legal_agreements/variables/{agreement_type} | |

# **legalAgreementsCreate**
> APILegalAgreement legalAgreementsCreate(legalAgreementCreateRequest)


### Example

```typescript
import {
    LegalAgreementsApi,
    Configuration,
    LegalAgreementCreateRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new LegalAgreementsApi(configuration);

let agreementType: LegalAgreementTypeEnum; // (default to undefined)
let orgId: string; // (default to undefined)
let legalAgreementCreateRequest: LegalAgreementCreateRequest; //

const { status, data } = await apiInstance.legalAgreementsCreate(
    agreementType,
    orgId,
    legalAgreementCreateRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **legalAgreementCreateRequest** | **LegalAgreementCreateRequest**|  | |
| **agreementType** | **LegalAgreementTypeEnum** |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APILegalAgreement**

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

# **legalAgreementsDelete**
> legalAgreementsDelete()


### Example

```typescript
import {
    LegalAgreementsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new LegalAgreementsApi(configuration);

let agreementId: string; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.legalAgreementsDelete(
    agreementId,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **agreementId** | [**string**] |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


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

# **legalAgreementsList**
> Array<APILegalAgreement> legalAgreementsList()


### Example

```typescript
import {
    LegalAgreementsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new LegalAgreementsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.legalAgreementsList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APILegalAgreement>**

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

# **legalAgreementsListAgreementTypes**
> object legalAgreementsListAgreementTypes()


### Example

```typescript
import {
    LegalAgreementsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new LegalAgreementsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.legalAgreementsListAgreementTypes(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**object**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **legalAgreementsListVariables**
> Array<APILegalAgreementVariable> legalAgreementsListVariables()


### Example

```typescript
import {
    LegalAgreementsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new LegalAgreementsApi(configuration);

let agreementType: LegalAgreementTypeEnum; // (default to undefined)
let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.legalAgreementsListVariables(
    agreementType,
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **agreementType** | **LegalAgreementTypeEnum** |  | defaults to undefined|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APILegalAgreementVariable>**

### Authorization

[personal_token](../README.md#personal_token), [service_account_token](../README.md#service_account_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


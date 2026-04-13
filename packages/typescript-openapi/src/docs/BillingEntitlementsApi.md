# BillingEntitlementsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**billingEntitlementList**](#billingentitlementlist) | **GET** /api/org/{org_id}/billing/entitlements | |
|[**billingEntitlementTest**](#billingentitlementtest) | **POST** /api/org/{org_id}/billing/entitlements/test | |

# **billingEntitlementList**
> APIEntitlementInfo billingEntitlementList()


### Example

```typescript
import {
    BillingEntitlementsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingEntitlementsApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.billingEntitlementList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIEntitlementInfo**

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

# **billingEntitlementTest**
> boolean billingEntitlementTest(aPIEntitlementRequest)


### Example

```typescript
import {
    BillingEntitlementsApi,
    Configuration,
    APIEntitlementRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingEntitlementsApi(configuration);

let orgId: string; // (default to undefined)
let aPIEntitlementRequest: APIEntitlementRequest; //

const { status, data } = await apiInstance.billingEntitlementTest(
    orgId,
    aPIEntitlementRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **aPIEntitlementRequest** | **APIEntitlementRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**boolean**

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


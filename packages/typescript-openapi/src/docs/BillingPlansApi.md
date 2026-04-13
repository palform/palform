# BillingPlansApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**billingPlanCancel**](#billingplancancel) | **DELETE** /api/org/{org_id}/billing/plan/{stripe_subscription_id} | |
|[**billingPlanGet**](#billingplanget) | **GET** /api/org/{org_id}/billing/plan | |
|[**billingPlanInitiate**](#billingplaninitiate) | **POST** /api/org/{org_id}/billing/plan/initiate | |
|[**billingPlanList**](#billingplanlist) | **GET** /api/billing/plans | |
|[**billingPlanSwitch**](#billingplanswitch) | **POST** /api/org/{org_id}/billing/plan/switch | |

# **billingPlanCancel**
> billingPlanCancel(cancelPlanRequest)


### Example

```typescript
import {
    BillingPlansApi,
    Configuration,
    CancelPlanRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingPlansApi(configuration);

let orgId: string; // (default to undefined)
let stripeSubscriptionId: string; // (default to undefined)
let cancelPlanRequest: CancelPlanRequest; //

const { status, data } = await apiInstance.billingPlanCancel(
    orgId,
    stripeSubscriptionId,
    cancelPlanRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **cancelPlanRequest** | **CancelPlanRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **stripeSubscriptionId** | [**string**] |  | defaults to undefined|


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

# **billingPlanGet**
> Array<APIBillingSubscription> billingPlanGet()


### Example

```typescript
import {
    BillingPlansApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingPlansApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.billingPlanGet(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIBillingSubscription>**

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

# **billingPlanInitiate**
> string billingPlanInitiate(initiatePlanRequest)


### Example

```typescript
import {
    BillingPlansApi,
    Configuration,
    InitiatePlanRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingPlansApi(configuration);

let orgId: string; // (default to undefined)
let initiatePlanRequest: InitiatePlanRequest; //

const { status, data } = await apiInstance.billingPlanInitiate(
    orgId,
    initiatePlanRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **initiatePlanRequest** | **InitiatePlanRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**string**

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

# **billingPlanList**
> APIBillingCurrencyResponseForArrayOfAPIBillingPlan billingPlanList()


### Example

```typescript
import {
    BillingPlansApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingPlansApi(configuration);

let currency: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.billingPlanList(
    currency
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **currency** | [**string**] |  | (optional) defaults to undefined|


### Return type

**APIBillingCurrencyResponseForArrayOfAPIBillingPlan**

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

# **billingPlanSwitch**
> APIBillingUpcomingInvoice billingPlanSwitch(switchPlanRequest)


### Example

```typescript
import {
    BillingPlansApi,
    Configuration,
    SwitchPlanRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingPlansApi(configuration);

let orgId: string; // (default to undefined)
let dryRun: boolean; // (default to undefined)
let switchPlanRequest: SwitchPlanRequest; //

const { status, data } = await apiInstance.billingPlanSwitch(
    orgId,
    dryRun,
    switchPlanRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **switchPlanRequest** | **SwitchPlanRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|
| **dryRun** | [**boolean**] |  | defaults to undefined|


### Return type

**APIBillingUpcomingInvoice**

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


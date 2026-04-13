# BillingInvoicesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**billingInvoiceList**](#billinginvoicelist) | **GET** /api/org/{org_id}/billing/invoices | |
|[**billingInvoicePreview**](#billinginvoicepreview) | **GET** /api/org/{org_id}/billing/invoices/next | |

# **billingInvoiceList**
> Array<APIBillingInvoice> billingInvoiceList()


### Example

```typescript
import {
    BillingInvoicesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingInvoicesApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.billingInvoiceList(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**Array<APIBillingInvoice>**

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

# **billingInvoicePreview**
> APIBillingUpcomingInvoice billingInvoicePreview()


### Example

```typescript
import {
    BillingInvoicesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingInvoicesApi(configuration);

let orgId: string; // (default to undefined)
let stripeSubscriptionId: string; // (default to undefined)

const { status, data } = await apiInstance.billingInvoicePreview(
    orgId,
    stripeSubscriptionId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **stripeSubscriptionId** | [**string**] |  | defaults to undefined|


### Return type

**APIBillingUpcomingInvoice**

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


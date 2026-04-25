# BillingCustomersApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**billingCustomerGet**](#billingcustomerget) | **GET** /api/org/{org_id}/billing/customer | |
|[**billingCustomerUpdatePaymentMethod**](#billingcustomerupdatepaymentmethod) | **POST** /api/org/{org_id}/billing/customer/payment_method_update_link | |

# **billingCustomerGet**
> APIBillingCustomer billingCustomerGet()


### Example

```typescript
import {
    BillingCustomersApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingCustomersApi(configuration);

let orgId: string; // (default to undefined)

const { status, data } = await apiInstance.billingCustomerGet(
    orgId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**APIBillingCustomer**

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

# **billingCustomerUpdatePaymentMethod**
> string billingCustomerUpdatePaymentMethod(updatePaymentMethodRequest)


### Example

```typescript
import {
    BillingCustomersApi,
    Configuration,
    UpdatePaymentMethodRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new BillingCustomersApi(configuration);

let orgId: string; // (default to undefined)
let updatePaymentMethodRequest: UpdatePaymentMethodRequest; //

const { status, data } = await apiInstance.billingCustomerUpdatePaymentMethod(
    orgId,
    updatePaymentMethodRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updatePaymentMethodRequest** | **UpdatePaymentMethodRequest**|  | |
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


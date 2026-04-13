# APIBillingPlan


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **string** |  | [default to undefined]
**features** | **Array&lt;string&gt;** |  | [default to undefined]
**highlight** | **boolean** |  | [default to undefined]
**name** | **string** |  | [default to undefined]
**price_annually** | [**APIBillingPlanPrice**](APIBillingPlanPrice.md) |  | [default to undefined]
**price_monthly** | [**APIBillingPlanPrice**](APIBillingPlanPrice.md) |  | [default to undefined]
**stripe_product_id** | **string** |  | [default to undefined]

## Example

```typescript
import { APIBillingPlan } from './api';

const instance: APIBillingPlan = {
    currency,
    features,
    highlight,
    name,
    price_annually,
    price_monthly,
    stripe_product_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

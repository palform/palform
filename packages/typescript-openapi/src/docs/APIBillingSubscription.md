# APIBillingSubscription


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canceling_at_end** | **boolean** |  | [default to undefined]
**currency** | **string** |  | [default to undefined]
**is_trial** | **boolean** |  | [default to undefined]
**period_end** | **string** |  | [default to undefined]
**plan_frequency** | [**APIBillingSubscriptionFrequency**](APIBillingSubscriptionFrequency.md) |  | [default to undefined]
**plan_name** | **string** |  | [default to undefined]
**price** | [**APIBillingPlanPrice**](APIBillingPlanPrice.md) |  | [default to undefined]
**stripe_plan_price_id** | **string** |  | [default to undefined]
**stripe_plan_product_id** | **string** |  | [default to undefined]
**stripe_subscription_id** | **string** |  | [default to undefined]

## Example

```typescript
import { APIBillingSubscription } from './api';

const instance: APIBillingSubscription = {
    canceling_at_end,
    currency,
    is_trial,
    period_end,
    plan_frequency,
    plan_name,
    price,
    stripe_plan_price_id,
    stripe_plan_product_id,
    stripe_subscription_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

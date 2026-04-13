# APIBillingUpcomingInvoice


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_due** | **number** |  | [default to undefined]
**currency** | **string** |  | [default to undefined]
**ending_balance** | **number** |  | [default to undefined]
**invoice_date** | **string** |  | [default to undefined]
**lines** | [**Array&lt;APIBillingUpcomingInvoiceLine&gt;**](APIBillingUpcomingInvoiceLine.md) |  | [default to undefined]
**next_payment_attempt** | **string** |  | [default to undefined]
**period_end** | **string** |  | [default to undefined]
**period_start** | **string** |  | [default to undefined]
**promotions** | [**Array&lt;APIBillingUpcomingInvoicePromotion&gt;**](APIBillingUpcomingInvoicePromotion.md) |  | [default to undefined]
**starting_balance** | **number** |  | [default to undefined]
**tax_amount** | **number** |  | [default to undefined]
**total_amount** | **number** |  | [default to undefined]

## Example

```typescript
import { APIBillingUpcomingInvoice } from './api';

const instance: APIBillingUpcomingInvoice = {
    amount_due,
    currency,
    ending_balance,
    invoice_date,
    lines,
    next_payment_attempt,
    period_end,
    period_start,
    promotions,
    starting_balance,
    tax_amount,
    total_amount,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

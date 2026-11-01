# APIOrganisation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billing_allow_overage** | **boolean** | Whether to allow the organisation to go over its response limit and therefore be charged for additional responses. | [default to undefined]
**created_at** | **string** |  | [default to undefined]
**display_name** | **string** | The name of the organisation, visible to admins and anyone filling any of the organisation\&#39;s forms. | [default to undefined]
**id** | **string** |  | [default to undefined]
**subdomain** | **string** | Optionally, a subdomain used by the organisation for short form links and possibly for authentication via OIDC (if configured). | [optional] [default to undefined]
**uses_oidc** | **boolean** | Whether the organisation uses OIDC authentication. | [default to undefined]

## Example

```typescript
import { APIOrganisation } from './api';

const instance: APIOrganisation = {
    billing_allow_overage,
    created_at,
    display_name,
    id,
    subdomain,
    uses_oidc,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

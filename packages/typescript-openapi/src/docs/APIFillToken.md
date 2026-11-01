# APIFillToken

A token with which end-users can access form filling related API endpoints for a specific form only.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**expires_at** | **string** | Optionally, an expiry time after which the token will no longer work | [optional] [default to undefined]
**form_id** | **string** |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**nickname** | **string** | An internal nickname to allow segmenting submissions by fill access token | [default to undefined]
**short_link** | **string** | Optionally, a shortened path through which the form can be filled. This requires the organisation to have a subdomain configured. The form will be available at &#x60;https://&lt;subdomain&gt;.palform.app/&lt;short_link&gt;&#x60;. Fill access tokens with short links can still be accessed at the long path, too. | [optional] [default to undefined]

## Example

```typescript
import { APIFillToken } from './api';

const instance: APIFillToken = {
    created_at,
    expires_at,
    form_id,
    id,
    nickname,
    short_link,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

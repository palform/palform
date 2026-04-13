# AuthCallbackRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_code** | **string** | The authorization code provided in response from the OIDC provider | [default to undefined]
**nonce** | **string** | The nonce provided when starting the auth flow | [default to undefined]
**redirect_url** | **string** | The redirect URL set by the client when starting the auth flow | [default to undefined]

## Example

```typescript
import { AuthCallbackRequest } from './api';

const instance: AuthCallbackRequest = {
    auth_code,
    nonce,
    redirect_url,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

# NewTokenRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expires_in_seconds** | **number** | Optionally, an expiry time after which the token will no longer work | [optional] [default to undefined]
**nickname** | **string** | An internal nickname to allow segmenting submissions by fill access token | [default to undefined]
**short_link** | **string** | Optionally, a shortened path through which the form can be filled. This requires the organisation to have a subdomain configured. The form will be available at &#x60;https://&lt;subdomain&gt;.palform.app/&lt;short_link&gt;&#x60;. Fill access tokens with short links can still be accessed at the long path, too. | [optional] [default to undefined]

## Example

```typescript
import { NewTokenRequest } from './api';

const instance: NewTokenRequest = {
    expires_in_seconds,
    nickname,
    short_link,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

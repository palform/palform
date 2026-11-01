# APIUserKey

A backend representation of a user\'s encryption key.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**expires_at** | **string** |  | [default to undefined]
**has_backup** | **boolean** | Whether the key is backed up to the DB in encrypted form | [default to undefined]
**id** | **string** |  | [default to undefined]
**key_pem** | **string** | PGP public key in ASCII-armored form | [default to undefined]
**user_id** | **string** |  | [default to undefined]

## Example

```typescript
import { APIUserKey } from './api';

const instance: APIUserKey = {
    created_at,
    expires_at,
    has_backup,
    id,
    key_pem,
    user_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

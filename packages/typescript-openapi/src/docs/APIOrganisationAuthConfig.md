# APIOrganisationAuthConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **string** |  | [default to undefined]
**client_secret** | **string** |  | [default to undefined]
**oidc_discovery_url** | **string** |  | [default to undefined]
**revoke_team_mappings** | **boolean** |  | [optional] [default to undefined]
**team_mapping_field_name** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { APIOrganisationAuthConfig } from './api';

const instance: APIOrganisationAuthConfig = {
    client_id,
    client_secret,
    oidc_discovery_url,
    revoke_team_mappings,
    team_mapping_field_name,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

# APIOrganisationDeletionRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**deletion_at** | **string** |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**include_user** | **boolean** |  | [default to undefined]
**organisation_id** | **string** |  | [default to undefined]
**reason** | [**CancelPlanRequestReason**](CancelPlanRequestReason.md) |  | [default to undefined]
**status** | [**OrganisationDeletionRequestStatusEnum**](OrganisationDeletionRequestStatusEnum.md) |  | [default to undefined]
**user_id** | **string** |  | [default to undefined]

## Example

```typescript
import { APIOrganisationDeletionRequest } from './api';

const instance: APIOrganisationDeletionRequest = {
    created_at,
    deletion_at,
    id,
    include_user,
    organisation_id,
    reason,
    status,
    user_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

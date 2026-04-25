# APIAuditLogEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**note** | **string** |  | [optional] [default to undefined]
**target_resource_id** | **string** |  | [default to undefined]
**target_resource_parent_ids** | **Array&lt;string&gt;** |  | [default to undefined]
**target_resource_type** | [**AuditLogTargetResourceEnum**](AuditLogTargetResourceEnum.md) |  | [default to undefined]
**user_display_name** | **string** |  | [optional] [default to undefined]
**user_id** | **string** |  | [default to undefined]
**verb** | [**AuditLogVerbEnum**](AuditLogVerbEnum.md) |  | [default to undefined]

## Example

```typescript
import { APIAuditLogEntry } from './api';

const instance: APIAuditLogEntry = {
    created_at,
    id,
    note,
    target_resource_id,
    target_resource_parent_ids,
    target_resource_type,
    user_display_name,
    user_id,
    verb,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

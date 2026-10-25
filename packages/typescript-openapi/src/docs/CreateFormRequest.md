# CreateFormRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**branding_id** | **string** | The branding to use. Will use Palform\&#39;s default look if no branding is provided. | [optional] [default to undefined]
**editor_name** | **string** | A name shown only to the admins and editors of the form | [default to undefined]
**in_team** | **string** | The team to create the form in | [default to undefined]
**one_question_per_page** | **boolean** | Whether to use the One Question Per Page (OQPP) flow, where each question is on a different page and the user has to click \&quot;Next\&quot; to continue. Otherwise, multiple questions are allowed per page, and the form editor can decide which pages contain which questions. | [default to undefined]
**title** | **string** | A public-facing name, shown only on non-OQPP forms | [default to undefined]

## Example

```typescript
import { CreateFormRequest } from './api';

const instance: CreateFormRequest = {
    branding_id,
    editor_name,
    in_team,
    one_question_per_page,
    title,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

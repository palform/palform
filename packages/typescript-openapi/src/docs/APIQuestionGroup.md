# APIQuestionGroup

A question group corresponds to a page.  On a one-question-per-page form, each group contains exactly one question, since each question is on its own page. On non-OQPP forms, each group can contain any number of questions. When filling the form, the user will have to click \"Next\" to continue to the next question group.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **string** | A user-facing description. Only shown on non-OQPP forms. | [optional] [default to undefined]
**id** | **string** |  | [default to undefined]
**step_strategy** | [**APIQuestionGroupStepStrategy**](APIQuestionGroupStepStrategy.md) |  | [default to undefined]
**title** | **string** | A user-facing title. Only shown on non-OQPP forms. | [optional] [default to undefined]

## Example

```typescript
import { APIQuestionGroup } from './api';

const instance: APIQuestionGroup = {
    description,
    id,
    step_strategy,
    title,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

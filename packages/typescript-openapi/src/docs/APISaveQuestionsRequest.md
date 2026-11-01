# APISaveQuestionsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**groups** | [**Array&lt;APIQuestionGroup&gt;**](APIQuestionGroup.md) | The full list of question groups. | [default to undefined]
**questions** | [**Array&lt;APIQuestion&gt;**](APIQuestion.md) | The full list of questions. Their group ID must be one of the specified groups. | [default to undefined]

## Example

```typescript
import { APISaveQuestionsRequest } from './api';

const instance: APISaveQuestionsRequest = {
    groups,
    questions,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

# APIQuestion


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration** | [**APIQuestionConfiguration**](APIQuestionConfiguration.md) |  | [default to undefined]
**description** | **string** | A Markdown-enabled description shown below the title. | [optional] [default to undefined]
**group_id** | **string** |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**internal_name** | **string** | The name of the field as used in bulk submission exports and recall tags. | [optional] [default to undefined]
**required** | **boolean** | Whether to prevent the question group being finished if this question has not been answered. Each question type has a different definition of what it means to be answered. | [default to undefined]
**title** | **string** |  | [default to undefined]

## Example

```typescript
import { APIQuestion } from './api';

const instance: APIQuestion = {
    configuration,
    description,
    group_id,
    id,
    internal_name,
    required,
    title,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

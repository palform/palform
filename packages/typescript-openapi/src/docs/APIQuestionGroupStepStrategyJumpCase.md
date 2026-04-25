# APIQuestionGroupStepStrategyJumpCase


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**APIQuestionGroupStepStrategyJumpCaseConditionList**](APIQuestionGroupStepStrategyJumpCaseConditionList.md) |  | [default to undefined]
**target_group_id** | **string** | If Some(uuid) jump to the group with that &#x60;uuid&#x60;. If None, submit the form. | [optional] [default to undefined]

## Example

```typescript
import { APIQuestionGroupStepStrategyJumpCase } from './api';

const instance: APIQuestionGroupStepStrategyJumpCase = {
    conditions,
    target_group_id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

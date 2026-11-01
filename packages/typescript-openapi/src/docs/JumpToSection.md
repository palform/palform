# JumpToSection

Jump to one of a list of sections. Each jump case is evaluated one-by-one in order, and the first one to match will be selected as the next question group to take the user to. If none of them match, the action will be to submit the form. Specifying an empty list here will mean that the action is always to submit the form.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JumpToSection** | [**Array&lt;APIQuestionGroupStepStrategyJumpCase&gt;**](APIQuestionGroupStepStrategyJumpCase.md) |  | [default to undefined]

## Example

```typescript
import { JumpToSection } from './api';

const instance: JumpToSection = {
    JumpToSection,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

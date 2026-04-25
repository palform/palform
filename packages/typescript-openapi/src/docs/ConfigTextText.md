# ConfigTextText


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_long** | **boolean** | Will use a textarea instead of an input | [default to undefined]
**validator** | [**APIQuestionTextValidator**](APIQuestionTextValidator.md) | Use client-side validation to ensure a valid input. Not validated server-side due to encryption, so this doesn\&#39;t guarantee valid data. | [optional] [default to undefined]

## Example

```typescript
import { ConfigTextText } from './api';

const instance: ConfigTextText = {
    is_long,
    validator,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

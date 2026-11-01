# APISubmissionStream


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | **Array&lt;string&gt;** | IDs of submissions that were deleted | [default to undefined]
**_new** | [**Array&lt;APISubmission&gt;**](APISubmission.md) | New submissions | [default to undefined]
**since** | **string** | If requested, the date since which the changes cover | [optional] [default to undefined]
**total** | **number** | The total number of non-deleted submissions to the form (over all time, regardless of &#x60;since&#x60;) | [default to undefined]

## Example

```typescript
import { APISubmissionStream } from './api';

const instance: APISubmissionStream = {
    deleted,
    _new,
    since,
    total,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

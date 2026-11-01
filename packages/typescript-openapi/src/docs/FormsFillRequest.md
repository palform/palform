# FormsFillRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | **string** | The submission data as an encrypted PGP message in armored ASCII form. The message itself must be a JSON object of the form &#x60;[palform_client_common::form_management::submission::InProgressSubmission]&#x60;. | [default to undefined]

## Example

```typescript
import { FormsFillRequest } from './api';

const instance: FormsFillRequest = {
    data,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

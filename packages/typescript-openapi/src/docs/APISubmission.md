# APISubmission


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**data** | **string** | The submission data as an encrypted PGP message in armored ASCII form. The message itself is a JSON object of the form &#x60;[palform_client_common::form_management::submission::InProgressSubmission]&#x60;. | [default to undefined]
**for_token** | **string** | The fill access token that the user used to fill out the form | [optional] [default to undefined]
**id** | **string** |  | [default to undefined]

## Example

```typescript
import { APISubmission } from './api';

const instance: APISubmission = {
    created_at,
    data,
    for_token,
    id,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

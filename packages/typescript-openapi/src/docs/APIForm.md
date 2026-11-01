# APIForm


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_delete_submission_after_days** | **number** | Submissions are automatically deleted this number of days after submission | [optional] [default to undefined]
**branding_id** | **string** |  | [optional] [default to undefined]
**editor_name** | **string** | A name shown only to the admins and editors of the form | [default to undefined]
**enable_captcha** | **boolean** | Whether to require a captcha to submit the form | [default to undefined]
**end_configuration** | [**APIFormEndConfiguration**](APIFormEndConfiguration.md) |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**notification_email** | **boolean** | The email address to which new response notifications are sent | [default to undefined]
**notification_webhook_url** | **string** | The URL of the webhook to which new response notifications are sent | [optional] [default to undefined]
**one_question_per_page** | **boolean** | Whether to use the One Question Per Page (OQPP) flow, where each question is on a different page and the user has to click \&quot;Next\&quot; to continue. Otherwise, multiple questions are allowed per page, and the form editor can decide which pages contain which questions. | [default to undefined]
**response_count** | **number** | An up-to-date count of all non-deleted responses across all fill access tokens | [default to undefined]
**team_id** | **string** |  | [default to undefined]
**title** | **string** | A public-facing name, shown only on non-OQPP forms | [optional] [default to undefined]

## Example

```typescript
import { APIForm } from './api';

const instance: APIForm = {
    auto_delete_submission_after_days,
    branding_id,
    editor_name,
    enable_captcha,
    end_configuration,
    id,
    notification_email,
    notification_webhook_url,
    one_question_per_page,
    response_count,
    team_id,
    title,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

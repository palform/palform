# APIFormEndConfiguration

A configuration for what the page shown to the user after successfully submitting the form should look like

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **string** | A Markdown-enabled message to show to the user | [optional] [default to undefined]
**redirect_to** | **string** | If specified, a button is shown with the text \&quot;Continue\&quot; that links to this URL | [optional] [default to undefined]
**show_restart** | **boolean** | Whether to show a button to start a new submission from scratch | [default to undefined]

## Example

```typescript
import { APIFormEndConfiguration } from './api';

const instance: APIFormEndConfiguration = {
    message,
    redirect_to,
    show_restart,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

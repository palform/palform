---
sidebar_position: 2
---

# Webhooks

You can configure your form to automatically send a webhook notification response is received.
Since Palform is end-to-end encrypted, the webhook payload includes the encrypted PGP message of the response, which your application can decrypt using the large ecosystem of [language-specific OpenPGP libraries](https://www.openpgp.org/software/developer/).

## Creating a webhook

1. Open your form and select the **Settings** tab
2. Under **Response notifications**, click **New webhook**
3. Enter the publicly-accessible HTTPS URL of your application to which you want requests to be delivered
4. Click **Add**. A simple TCP connection will be made to ensure it works.
5. Copy the signing secret, as this will not be shown again.

That's it! All future requests will now result in a request to your specified endpoint.

You can register multiple webhooks. On each response, a request will be delivered to all of your configured webhooks.

## Request payload

Webhooks are delivered as `POST` requests with `Content-Type: application/json`.
The body is a JSON object with the following keys:

- `submission_id` (`string`): The ID of the newly created submission
- `form_id` (`string`): The ID of the form to which the submission was made
- `created_at` (`string`): ISO 8601 formatted timestamp of the submission time
- `payload` (`string`): ASCII-armored PGP message containing the submission data as a JSON in the shape of [`InProgressSubmission`](https://github.com/palform/palform/blob/7d70ae8491dd43d2174be4907610c0072fae37bb/packages/client-common/src/form_management/submission.rs#L17)

:::warning

While the JSON value inside `payload` is expected to be a valid `InProgressSubmission`, Palform has no way of validating encrypted data on the backend. The contents of the payload **must not be trusted** and need to be handled with care. Additionally, validation of form fields (e.g. to ensure the contents of a text input are a valid email address) is **only done on the frontend**, so you need to validate them again after decryption.

:::

## Signature

The raw JSON bodies of webhook requests are signed using the HMAC-SHA256 algorithm with the signing secret generated when you registered the webhook.
The resulting hex-formatted signature is included in the `X-Palform-Signature` header of the request.

It is **highly recommended** to verify the payload against the signature before doing any other processing.
This way you can be sure the request came from Palform itself, since no-one else knows your signing secret.

## Monitoring webhook status

In your form's **Settings** tab, you can monitor the status of your configured webhooks.
After each delivery attempt, a log is made, along with details about whether the attempt was successful or the error that occurred.
If the latest attempt was unsuccessful, the webhook will show as **Unhealthy**.
Click on the health status to view all details about the delivery attempts.

## Delivery frequency

Webhook deliveries are attempted approximately once a minute. This means it can take up to 60 seconds after a form submission for the webhook to be sent.

Failed delivery attempts will be repeated every minute until one succeeds.

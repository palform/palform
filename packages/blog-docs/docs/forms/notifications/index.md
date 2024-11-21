---
sidebar_position: 4
---

# Response notifications

Due to Palform's strict end-to-end encryption, options for being notified of responses are somewhat limited. This protects your security and is a strong way to prevent data breaches.

There are currently two ways to get notified: emails and webhooks.

## Emails

When email notifications are enabled for your form, everyone in the team to which the form belongs will receive an email for each response. These emails will be delivered synchronously as the response is made.

The email will not contain any specific details about the response such as answers to questions, as these details are not available to the server sending the message.

To enable emails, open the relevant form and select the **Settings** tab, scroll down, and enable the **Email** option.

## Webhooks

You can choose to receive an HTTP POST request to your specified endpoint for each form response. These requests will be made asynchronously through a task queue, meaning it can take around 1-2 minutes after the submission.

You can specify any number of endpoints for a form, and requests will be made to all endpoints for all requests. To be eligible, endpoints must:

- Use the `https` scheme
- Be publicly accessible from our servers in the EU
- Be able to take TCP connections
- Respond to `POST` requests

To create a webhook, open the relevant form and select the **Settings** tab, scroll down, and click **New webhook**. Enter the endpoint URL, click **Add**, and make sure to note down the signing secret.

### Message format
The HTTP POST request will be made with a JSON-encoded body in the following format:

```json
{
    "submission_id": "string",
    "form_id": "string",
    "created_at": "iso_8601",
    "payload": "string"
}
```

The payload is a PEM-encoded OpenPGP message, encrypted using the private keys of all the team members of the team to which the form belongs. You can easily decrypt this server-side by using a [PGP library in your language](https://www.openpgp.org/software/developer/). Simply [create an additional key](/keys/create) in your Palform account and store the private component on your server, ready for decrypting messages.

### Request signing
To verify that requests are genuinely from Palform's servers, you can optionally check the request's signature using signing secret provided when you created the webhook.

The request body is signed using HMAC SHA256, and the signature is sent as the `X-Palform-Signature` header. To verify the request:

1. Read the value of `X-Palform-Signature`.
2. Read the full body of the request and run it through the HMAC SHA256 algorithm with the webhook signing secret as the key.
3. Compare your generated key to the header value.
4. If they match, the request is authentic!

To keep this process secure, it's important you don't share the signing secret with anyone. Never include it in source repositories and take care when transferring it between servers.

If you lose the secret, simply delete and re-add the webhook to the form.

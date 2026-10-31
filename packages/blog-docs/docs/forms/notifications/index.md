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

To read more, see our [Webhooks documentation](../../api/webhooks.md).

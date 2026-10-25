---
sidebar_position: 1
---

# Getting started with the API

:::note

API access is only available on the Business plan.

:::

Palform's API lets you access the platform programmatically and supports nearly everything that you can do as a human via the dashboard.
It's the exact same API that the dashboard itself uses, so you can be sure that all the latest features will be available.

The API is fully specified via OpenAPI and many endpoints have detailed documentation, with more being documented as often as possible.

This guide will explain how to get started.

## Endpoints

The API is available at **https://api.palform.app**.
Using it is as simple as making plain RESTful HTTP requests with JSON.

## Authentication

To use the API, you need to create a **service account**.
This is essentially a regular Palform user that can join organisations and make changes just as a human user would.
But instead of a username and password, it has a **service account token**, which you specify in each request you make to the API in order to authenticate yourself.

To create a service account:

1. Open your organisation

2. Click **Organisation** > **Service accounts**

3. Click **Add new service account**

Once you've created it, remember to save the token somewhere safe. You won't be able to see it again!

:::warning

Treat the service account token like a password: do not share it and don't commit it to a repository.

:::

When making requests, include the service account's token in the `Authorization` header:

```http
POST https://api.palform.app/api/...
Authorization: Bearer <service_account_token>
```

## Resources

Palform is made up of a number of resource types which are important to understand when building any integration using the API.
Some of the most important ones are:

- **Organisations**: the main resource under which all teams, forms, and more are contained. Organisations subscribe to a billing plan (or stay on the free plan) and all subscription benefits are calculated on the organisation level.
- **Users**: can log in and perform administrative actions. A user can be a member of multiple organisations. Service accounts are users too, but they cannot be a member of more than one organisation.
- **Teams**: a logical grouping of forms, branding configurations, and members with various permissions. An organisation always has a **default** team and all members of the organisation are always members of its default team. Organisations can have multiple teams, and users can be members of multiple teams. Forms belong to teams, and when a form response is submitted, it is encrypted with the keys of all of the members of the form's team.
- **Forms**
- **Question groups**: a logical grouping of questions within a form. A form can have multiple question groups. They are presented in the UI as a page.
- **Questions**: each question group has at least one question. On one-question-per-page forms, question groups always have exactly one question, while on regular forms they can have multiple.
- **Submissions**/responses: encrypted response data to the questions in a form

### Resource IDs

Each resource has an ID with a prefix depending on the resource type, which allows unambiguously identifying a resource solely based on its ID.
For example, the ID `org_0PYD2XSGB73PT` identifies an organisation.

In the API reference, resource-specific IDs are denoted with a schema named `TSID_<prefix>`, such as `TSID_org`. These are actually strings, but the OpenAPI spec uses the specific types to make it extra clear which resource's ID is being referenced.

A full list of entities and their corresponding prefixes is available in [Palform's source code](https://github.com/palform/palform/blob/main/packages/tsid/src/resources.rs).

## Permissions

Service accounts behave exactly like human users, so their permissions are just as granular, too.
To give your service account access to specific actions, you need to add it to the relevant teams with the correct role, and in some cases make it an organisation-wide admin.

See [Teams and permissions](../orgs/teams.md) for more details.

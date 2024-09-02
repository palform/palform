---
sidebar_position: 0
---

# Features explained

In this guide, we'll explain what each feature and quota means, and what their behaviour is.

For an overview of the features included in each plan, please see the Pricing section of [our home page](https://palform.app).

## Response count
Plans are advertised with a given _responses/month_ limit. This can behave differently, depending on whether you have opted in to _response overage_.

**When using monthly billing**, it's really simple! This number is the limit of responses you're allowed to receive each month. For example, your limit could be 5,000 responses per month. The 5,001st response will either be blocked with a user-facing error message, or alternatively accepted for an additional charge.

**When using annual billing**, the behaviour is the same, except the limit is per _year_ instead. For example, if your plan states a limit of 5,000 responses per month, we'll actually account for 60,000 responses per year. We won't care how you distribute the responses throughout the year. You could, for example, use all 60,000 responses in a single month if you want!

### Response overage
This is an **opt-in** behaviour, off by default.

- When turned **off**, new responses once you've reached your limit will be **blocked with a user-facing error message**. This way, you won't incur any additional charges beyond the price of your plan.

- When turned **on**, we'll charge for new responses at a fixed rate of **£1/€1.15/$1.20** per 2,000 additional responses (regardless of your plan).

  - Some customers may have a specific use case requiring fewer features and a very large number of responses. If this sounds like you, please [contact us](/), and we'll try work out a discounted overage rate.

Response overage cannot be turned on on the Free plan. Responses are always counted per-month (rather than per-year) on the Free plan.

:::warning
If you activate response overage, incur some costs, then deactivate response overage within the same billing cycle, you will still be charged for the costs you incurred. However, new responses from the point of deactivation will be blocked, so you won't incur any _further_ costs.
:::

## User count
This refers to the number of users you are allowed to have in the entire organisation at any given time.

All users in your organisation, in all teams, are counted; including the original creator of the organisation.

You can assign each user to as many teams as you want.

## Team count
This is the number of teams you are allowed to have in your organisation at any given time.

## Exporting submissions
If available, this allows you to generate machine-readable format exports of all submissions of a given form. The export happens on the client side, keeping the security guarantees of our end-to-end encryption.

There are no limits as to how many exports you can create!

## Palform attribution
By default, all Palforms have our purple branding, and an attribution crediting Palform. It's a relatively subtle mark that's not meant to cause too much annoyance to your end users:

![Example of the attribution mark. A small purple link "Create secure forms for free with Palform", followed by a smaller gray text "Copyright 2024 Palform Limited, registered company 15796859"](./attribution.png)

On upgraded plans, you can choose to remove this message by **creating a custom branding scheme**. This is really easy to do, and you don't have to have any design knowledge or artistic skill. By default, custom branding schemes look similar to the non-custom design of our forms.

If you want, you can choose to still include the attribution in a custom scheme.

## Import your own keys
Normally, we allow only generating new keys through the Palform dashboard. With this feature, each user in your organisation will _also_ be able to import existing OpenPGP keys by simply uploading their ascii-armored form.

More details are available in our [key import documentation](/keys/create#importing-a-key-advanced).

## Dedicated subdomain
A super-useful tool for memorable branding, dedicated subdomains give your brand a clearly identifiable personal space on our website, making it easy for your users to type in links to your forms.

You'll choose a subdomain `my-custom-text.palform.app`, and then you'll be able to create links to your form fills by adding a custom "short link" to your Share Tokens.

For example: `my-custom-text.palform.app/spring2024` could be a link to your form!

If you have this feature enabled on your plan, you can choose one subdomain (which cannot be deleted or modified without contacting our support team). Then you'll be able to create an unlimited number of unique short links.

## Captcha
By default, anyone can fill in your form responses, including bots. While unlikely, this may mean you start receiving spam answers, which can clog up your form responses and be annoying when you're trying to analyse your results.

By enabling a captcha, you can dramatically reduce the number of spam responses, by requiring non-robot validation on respondents. Unlike Google reCAPTCHA, this is done in a non-intrusive way, without silly picture prompts and other annoying features. Your genuine users will continue to receive a smooth experience, while robots are blocked efficiently.

There is no limit to the number of forms you can activate captchas on, or the number of responses submitted with captcha validation.

We don't store any information about captcha sessions, preserving the privacy of your responders.

## Response auto-deletion
To ease GDPR compliance, this feature allows you to set an automated deletion deadline for all Palform responses.

This feature is not limited to any number of forms or responses, and you can configure it freely. It will be turned off by default for any new forms you create.

## Audit logs
An unlimited number of logs within the last 30 days are stored and available in your organisation dashboard. Logs older than 30 days will be periodically cleared out by our system, and will not be available to view or download.

Logs are produced for various events within your organisation, but generally only for updating/creating/deleting operations, and not for viewing operations. More details are available in the [audit log documentation](/orgs/audit).

## Priority support
This gives a guaranteed first response time of 24 hours to any new support query you submit to our team, through one of our official sources [listed on our contact page](/).

Our guarantee gives you partial refunds if we violate our promise, as explained [in the compensation section](/#priority-support-guarantee).

## OpenID Authentication
A popular enterprise feature, this allows you to connect your own OIDC provider (e.g. Google Workspace, Microsoft 365, Gitlab, etc.)

Users will then sign in using your provider instead of our own authentication system, removing the need to maintain separate Palform accounts.

You can even provision team mappings, placing new and existing OIDC users into teams automatically based on a variable returned from your OIDC provider.

There is no limit to how many users you may provision through OIDC (except the total user limit, which is unlimited for our Business plan). There is also no limit to how many team-user mappings you can create.

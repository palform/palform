---
authors: [pal]
---

# Encrypted surveys: Palform's security practices

[Palform](https://palform.app)'s aim is to do for forms what [Proton](https://proton.me), [Tuta](https://tuta.com), and others did for emails: make them **encrypted, privacy-respecting, and open**. And most of all, to be so simple that anyone can use it, without an understanding of the technology.

It's an ambitious project, and I'm well aware that transparency is critical. I've implemented a range of security practices to ensure Palform can deliver on these claims. However, there's also a range of compromises I've had to make to ensure a good balance between usability and security.

This post aims to lay out clearly what Palform protects you against and how, and also what it _doesn't_ protect you against.

<!-- truncate -->

## Security practices

### End-to-end encrypted responses

Palform end-to-end encrypts all responses to forms, meaning no intermediaries (Palform, our server providers, infrastructure operators, etc.) are able to see them. This is always on (unlike e.g. [Jotform](https://www.jotform.com/help/344-encrypted-forms-and-how-to-use-them/) where it's an opt-in feature) and cannot be turned off. This helps reassure users that when they see a Palform, it will be encrypted.

Correctly implemented encryption is one of the most powerful methods of safeguarding data. It's something no major form builders provide by default.

Palform uses OpenPGP through the [Sequoia PGP](https://sequoia-pgp.org/) library (in Rust). Compared to other implementations, this is written in a memory-safe language (automatically eliminating whole categories of vulnerabilities) while still being maintained very actively.

<details>
  <summary>How does your encryption work?</summary>

- Unlike other E2EE platforms, keys are _not_ linked to user passwords. Instead, we create a random OpenPGP certificate for you in your browser and save it in long-term browser storage.
  - Where possible, we instruct your browser to never delete this data, even when it's tight on space.
- We generate a random passphrase made of a series of English words, and upload your full certificate to the server, encrypted with this passphrase.
- Separately, the public component of the certificate is uploaded to the server unencrypted.
- Users can be in teams, and each form belongs to a team. Multiple users need to see the responses to a form.
- When the form is being filled, the server sends the public keys of all users in the team the form belongs to, and the client encrypts with these.

You can read more [details in our documentation](/keys/technical).

</details>

<details>
  <summary>Do keys expire?</summary>

By default, we create non-expiring keys (for technical reasons, they actually expire in 100+ years). You can manually override the expiration date if you'd like.

We keep your expired keys indefinitely to let you decrypt responses made with them. New responses are only made to in-date keys. If you have none, new responses will be refused with a user-facing error message.

</details>

<details>
  <summary>I thought OpenPGP was dead?</summary>

Firstly, it's not. While adoption has been very limited for personal users, services with Proton Mail bundle it in an online interface where most people don't even know they're using it. As a result, it's actually deployed very very widely.

Secondly, it's a highly-trusted standard with an enormous community behind it, loads of documentation, first-class support in most languages, etc. Many users already use some form of OpenPGP and have existing keys they might want to use (you can import your own keys on Palform!), as well as an overall trust in its highly-proven ability to secure crucial data.

I don't want Palform to be a walled garden, and relying on something portable is a good way of ensuring that.

Also, Palform doesn't implement all features of OpenPGP, just the encryption. At the moment, it doesn't use signing as it doesn't really apply anywhere. The code implementation of the algorithms is intentionally extremely simple, relying only on the most simple highly-maintained functions of OpenPGP.

</details>

### Open source code base

The entirety of Palform is [open source](https://github.com/palform/palform), including the complete backend/frontend, the landing page, and even this blog post. I develop it fully in public, with no "hidden" code.

I know, [it's not guaranteed secure just because it's open source](https://www.privacyguides.org/en/basics/common-misconceptions/#open-source-software-is-always-secure-or-proprietary-software-is-more-secure).

However, it's a step in the right direction. As I mentioned in [a previous post](/blog/2024/09/02/open-source), the client-side part of Palform performs a lot of encryption work in opaque binary WASM files (primarily for performance reasons). Having to murkily reverse engineer these would massively hinder users' confidence in the cryptography being applied correctly.

Having a public changelog also increases accountability and transparency into everything we do.

I'm also working on a self-hosted version, which I'll be posting updates on soon!

### Third-party providers

There's certain elements of Palform that I can't reinvent from scratch. Right now, it uses third-party services for these components:

- **Email notifications** ([Mailgun](https://www.mailgun.com/))
  - Your name and email address are passed to Mailgun, as well as the names of your forms when there's a response.
  - We never pass encrypted or plaintext response data to Mailgun
- **Captcha** ([Cloudflare Turnstile](https://www.cloudflare.com/en-gb/products/turnstile/))
  - This is more private and less annoying than e.g. Google reCAPTCHA.
  - It's only loaded on pages where it's needed. By default it's not loaded on form fill pages, unless the form captcha feature is enabled.
- **Analytics** ([Plausible Analytics](https://plausible.io/))
  - This is only loaded on the landing page and blog/docs pages. It does not collect any personally identifiable information.
- **Payments** ([Stripe](https://stripe.com))
- **Server and database hosting** ([Scaleway](https://www.scaleway.com/en/))

Other than this, everything is built into the codebase. Reducing dependence on third-parties helps the open source nature of Palform and will make it easier to set up self-hosting.

No plaintext response data is ever passed to any third-parties.

We don't track any of your personal info for advertising purposes, and we don't operate any ads on our websites.

### Data location

All data is stored and hosted in the EU on Scaleway's servers, and so is subject to the strict privacy regulations of the GDPR.

Palform is a company is based in the UK, and is subject to the Data Protection Act, which is currently very similar to the GDPR.

Our servers and databases only ever store encrypted response data. Plaintext responses never leave browsers.

### Data minimisation

As required by law, we only collect the bare minimum data we need to operate. We don't collect any additional data on your form responses either (e.g. IP addresses or other analytics). All we can tell is how many responses your forms had, and when they happened.

### Backups

Our database is automatically backed up every day, allowing us to quickly recover from corruption and data loss events. This helps us avoid losing your encrypted response data.

### Authentication & authorisation

You can use either email/password or OpenID Connect (on higher plans) to sign into Palform. We store passwords hashed with the industry-recommended Argon2 algorithms, and never in plaintext.

We support two-factor authentication with TOTP, and as a Yubikey user myself, I plan on adding security key support soon.

Authorisation is managed in a tiered system of organisations and teams. By default, new users in your org are added to your "default" team, with read-only permissions. You can customise this and set read, write, or admin permissions. Certain assets, like images and branding configurations, also belong to a team, and can be applied to all forms within that team.

This system means larger organisations also feel comfortable within Palform. They can even set up rules to automatically assign users into teams with givern permissions based on a response from their OIDC server.

### Audit logs

Larger organisations using Palform can make use of audit logs to track events that occur in their organisation. This is not intrusive of employee's actions, as it doesn't track reads, only writes.

Any significant events, such as configuration changes or form response deletions, are clearly shown in the logs with associated metadata.

### Other helpful features

I've tried to add features that help users improve their security practices and GDPR compliance:

- Auto-delete form responses within a certain number of days to comply with data retention limits
- Force include a Privacy Policy link in all forms in a team
- You own your data: responses can be exported in multiple formats whenever you want

## Limitations

Nothing is perfect, and it's very important to understand what Palform can't proctect you against.

Here are some known limitations:

- **Supply chain attacks**: while we follow modern standards to prevent them (e.g. transparent CI builds, pushing containers straight to Scaleway, etc), they're a category of attacks that are essentially impossible to completely eliminate.

  - For example, if someone compromised Palform's servers, they could send false public keys for an organisation, or serve up fake JavaScript on the frontend that steals encryption keys. This is the case for many end-to-end encrypted applications.

- **Backend validation**: our backend is unable to validate your form responses, because it can't see them. You have to trust our client-side validation, which can technically be overcome. Therefore, it's important to not rely 100% on Palform's validation rules.

- **Metadata**: by the nature of OpenPGP, the key IDs of users are included in the encrypted responses. This might make it possible to tell which organisation/team/user a response can be read by. Also, the time of submission and the access token used are stored unencrypted.

- **Non-response data**: questions, form details, administrative users, etc. all have their details stored unencrypted in our database.

- **You still have to comply with GDPR** and any other privacy regulations. We don't magically do that for you, but we try to publish tools that help.

## Reporting issues

We're currently too small to operate a paid bug bounty program. However, we would still really appreciate responsible disclosures of any potential security issues you find in our systems, no matter how big or small.

These can be related either to our production websites (\*.palform.app) or our code repository (https://github.com/palform/palform).

If you would like to report a problem, please email our [secure email address](/#encrypted-email).

## Questions and concerns

If you have any questions about our security practices, please [contact us](/) and we'll be happy to help!

---

> Thanks for reading! New here? Try out [Palform](https://palform.app) online with unlimited responses free of charge.

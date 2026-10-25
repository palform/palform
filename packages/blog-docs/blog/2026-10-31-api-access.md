---
authors: [pal]
---

# Bringing API access to Palform

Exciting news! One of Palform's most-requested features is finally here: **programmatic access**.
You're now able to do almost everything you can do in the dashboard, but via a neatly organised, fully OpenAPI-specified RESTful API.

<!-- truncate -->

:::note

API access is only available on the [**Business plan**](https://palform.app/#pricing).
Please consider upgrading to gain access and to support Palform's mission to spread privacy-respecting
data collection across the world.

:::

Starting right now, you can [set up a service account](/api/getting_started) in a few easy clicks and start playing around.
API access to Palform will open up exciting new possibilities for integrations and automations, and it'll be exciting to see what you do with it!

Authentication is done through **service accounts**, which behave just like regular users and are subject to the same permissions system as them. This ensures that the design feels intuitive to existing Palform users and allows a logical, granular management of access.

Form responses are and always have been encrypted with OpenPGP, which is one of the most widely used cryptographic frameworks in the industry.
The API (and the existing [webhook feature](/api/webhooks)) lets you leverage this by decrypting end-to-end encrypted data
straight on your own servers, using one of the [many language-specific OpenPGP libraries](https://www.openpgp.org/software/developer/) in existence.

Palform's backend is written in Rust, and I was already using the extremely helpful [Apistos](https://github.com/netwo-io/apistos) package to automatically generate an OpenAPI spec straight from the source code.
This spec is then converted into TypeScript code for our frontend by [OpenAPI-Generator](https://github.com/OpenAPITools/openapi-generator).

With some adaptations and improvements to documentation, this thoroughly-specified API is now available to Palform's users.
I will continue to add to the endpoint documentation depending on which features turn out to be most often used.

I hope you enjoy the new API! Of course, don't hesitate to [get in touch](/) if you need help or would like to share feedback.

This feature was added as a result of countless submissions to the new [feedback form](https://pf.palform.app/feedback). I'm currently working on a re-prioritised [roadmap](/roadmap) to better reflect what users really want. Stay tuned!

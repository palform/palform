---
authors: [pal]
---

# Complying with GDPR as a Google Forms user is pretty hard

I'm Pal, the founder of [Palform](https://palform.app). It's a free Google Forms alternative that basically eliminates all of the pains and sorrows I'm about to detail in this post. And it [doesn't cost anything](https://palform.app/#pricing).

<!-- truncate -->

:::warning
I'm by no means a GDPR expert, and this post is not legal advice. I've tried my best to research everything and provide evidence, but if I've made a mistake please email pal@palform.app.
:::

---

If you're a business based _anywhere in the world_ and you're processing the personal data of citizens of the UK, EU, or EEA, you're required to comply with the famous [GDPR](https://gdpr.algolia.com/).

> Example: Random Co. is a company based in California. They have a website with Google Analytics, meaning they're collecting personal information about users automatically. The website handles the personal information of EU users. Therefore, Random Co. must comply with the GDPR.

This goes for Google Forms, too. If you have any EU/EEA/UK citizens filling in your form, you _must_ comply.

So how do you do it?

## Requirements for compliance

I won't detail how to comply with the GDPR. That's a whole profession.

Two important requirements are:

- **[Transparency](https://gdpr.algolia.com/gdpr-article-12)**. You need to provide a Privacy Policy or similar document, that details exactly how you process personal information, safeguards for protection, and the rights your users have.

- **[Data location](https://gdpr.algolia.com/gdpr-article-44)**. Transferring the data of EU/UK citizens outside of the EU/UK (e.g. into US data centers) can involve additional complexity. For example, transferring data to the US is usually done by participating in the [Data Privacy Framework](https://www.dataprivacyframework.gov/key-requirements), which comes with several requirements. It's often easier to just keep data in the EU, unless you're a large corporation with specific requirements.

## Can I fulfill these requirements with Google Forms?

Yes! It is possible to use Google Forms in compliance with the GDPR, but there are additional steps you need to take.

### Transparency

Google Forms, by default, does not provide a way to display your organisation's Privacy Policy in forms you create. It's up to you to ensure _every single_ form created by anyone has a dedicated field linking to your data handling practices.

Google was [fined €50m in 2019](https://www.bbc.com/news/technology-46944696) for failing to provide the GDPR-mandated information, although in relation to Google Ads, not Forms. My point is, there are actual fines for failing to provide the correct information.

### Data location

You can configure the location of your data **only if you're subscribed to Google Workspace Business Standard** or higher. [Here's how to do it](https://support.google.com/a/answer/14310028?hl=en).

If you're not using Google Forms within a Workspace account (i.e. just through a personal Google account), **setting the location is not possible**.

If your location is not set, **your data is likely to be stored in the US**. You don't really get much control.

If your data is (or could potentially be) stored in the US, you _must_ comply with the EU-US Data Privacy Framework, or have another mechanism for data transfers in place. You can complete an [online certification](https://www.dataprivacyframework.gov/) to check your compliance.

## Other problems

**Sharing your forms** with collaborators can introduce complexity. It's easy to get a setting wrong and accidentally allow collaborators to re-share the form with more people, revealing the responses to people outside your organisation you shouldn't be able to see it.

You'll also need to ensure you're complying in terms of the **data you're collecting**. As with any data collection tool, you need to avoid collecting excessive data, and make special provisions for sensitive categories of personal information, such as medical records (including seemingly simple questions like "Do you have any dietary requirements?"). Of course, this goes for all form builders.

Finally, there's the question of **user trust**. Google has previously [misled users](https://www.bbc.com/news/technology-63635380) about their practices (this was not in relation to Forms, though). The majority of consumers worldwide are "[somewhat or very concerned](https://iapp.org/resources/article/privacy-and-consumer-trust-summary/)" about their privacy, and Google is arguably one of the reasons why.

## Making compliance a little easier

Here at [Palform](https://palform.app), we've built a privacy-focussed alternative to Google Forms. It aims to make GDPR compliance easier and increase customer trust. It's just as easy to use and it's completely free, too.

Here's how it helps:

- You can create **organisation-wide privacy policies** that automatically appear on all forms. This avoids costly mistakes and standardises the way you distribute information.

- **All data is stored exclusively in the EU** in secure databases. Plus, your responses are **end-to-end encrypted**, massively reducing the possibility and impact of data breaches, and helping you sleep better.

- You can view and manage your form sharing centrally, and responses cannot be shared outside of your organisation by default. Plus, each viewer has their own key, so you get guaranteed knowledge of who can access each individual response.

Of course, it's still important to comply with the GDPR in terms of the data you're collecting and the information you're providing. Palform is not a magic solution to all your problems, but it's here to help.

There's also other nifty features, like the customisable design tools and built-in analytical reports. All-in-all, it has all the benefits of Google Forms (and more), but without many of the drawbacks.

[Give it a try](https://palform.app) and see what you think!

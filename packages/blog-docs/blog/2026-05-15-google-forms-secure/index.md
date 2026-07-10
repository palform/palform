---
authors: [pal]
---

# How secure are Google Forms?

Data breaches and security vulnerabilities are becoming more and more commonplace.
Last year alone, [over 375 million people](https://privacyrights.org/resources-tools/reports/2025-data-breach-report) were in at least some way impacted by a data breach, and this number shows no signs of decreasing in the near future.

[Google Forms](https://www.about.google/forms/about/) is an incredibly widely used tool for collecting all sorts of data.
If you're a user of it, you'll want to take a detailed look at its security practices to see if end-user provided personal
information is being stored and handled as safely as it should be.

<!-- truncate -->

Being operated by a large widely-respected corporation, Google Forms is convenient and **does use solid data security practices**.
However, there are still some shortcomings and many ways for you to create **accidentally insecure forms**. We'll cover some of these
examples in this article.

## How Google Forms works

Google Forms is a very simple and convenient tool.
Coming as a built-in feature of the Google ecosystem (including their business-oriented [Google Workspace](https://workspace.google.com/) product), it's often the first survey-building tool that marketers, researchers, and just about everyone else reaches for.

Let's take a look at how it works behind the scenes:

- **Each Google Form is tied to an owner** by default. This can be customised when being used within a Google Workspace (e.g. the form can be owned by an organization instead of a person), but it's important to remember that the access restrictions are defined by the owner.

- **The questions and configuration are stored on Google's servers**. Everything about the form, including the number and types of question, as well as design customizations you may have made, is stored directly on Google's vast and reliable infrastructure. **Downtime is nearly non-existent**, meaning you can trust it to host forms that thousands of people need to fill in in a short window of time.

- **Form responses go straight to Google's databases**. When someone fills in your form, their response is stored in a Google datacenter. As a regular consumer, **you have no control of this datacenter's location** in most cases. The responses might be stored in the US, the EU, or even elsewhere. We spoke some more about this in [a dedicated article](../2024-09-07-google-forms-gdpr/index.md#data-location).

- **Customization is quite limited**. To keep the product simple, Google doesn't allow a great deal of customization with regards to design. What's more, with just 12 question types (of which some are nearly identical, so around 6 unique ones depending on how you count), the types of data you can acquire are also somewhat constrained. However, this helps Google Forms focus on optimizing security and performance across a narrower set of options.

## What makes it secure

Google applies some of the **strongest security practices in the industry** to your data, massively reducing the chance of it being subject to unauthorised access.

All user data, including your form responses, are **encrypted at rest** with [AES-256](https://docs.cloud.google.com/docs/security/encryption/default-encryption), a leading cryptographic algorithm.
This means that even in the highly unlikely event that someone were to compromise Google's servers, they would find it nearly impossible to access your data.

Not only that: Google runs a widely respected [**bug bounty program**](https://bughunters.google.com/), allowing security experts and hobbyists to search for vulnerabilities and get paid for finding them.
This creates a natural incentive for people to disclose bugs and allows Google to get them fixed before they become a problem.

Overall, it's clear that Google Forms complies with industry-wide norms, and is certainly a **safe place for your users to submit their data**.

## Common pitfalls

All this being said, however, it is entirely possible to accidentally create insecure surveys with Google Forms.

Due to a slightly confusing permission management system, there are many common mistakes people tend to make.

- **Permissions can be inherited via Google Drive**. Many users don't realise that each Google Form is represented as an item in your Google Drive; specifically the Drive of the user who created it (unless changed afterwards). Just like a Google Doc or Sheet, Forms can be moved from folder to folder. Each folder can be shared with a different set of people, and those sharing settings will be **inherited by the form**.

  Any users you've shared the form with will be able to access all responses and edit the form's contents.

  This can often be fully unintentional, and could mean that your form is shared with the wrong people.
  Therefore, it's important to carefully check the "Share" menu of each form before you start using it.

- **The linked spreadsheet can have different permissions**. Forms allows you to link a Google Sheet, which will be continuously populated with new form responses. This spreadsheet is also subject to the same sharing configuration system as mentioned above, but the set of people it is shared with is not automatically synced to that of the form itself. For example, one group of people could have access to the form and a different group of people to the spreadsheet.

  It goes without saying that anyone with access to the spreadsheet can see all of the form responses.
  Make sure to be extra careful when using this feature and take great care when checking who has access to what.

- **There's a lack of granular access controls**. Whether a user has access to your form is sort of "all or nothing". Either they can't do anything (besides submit a response to the form if they have the link), or they can do everything: see responses, manage questions, and even delete data.

  In the "Share" menu, there's a small hidden setting to configure whether users who have access to the form can also edit the sharing permissions, but that's it.
  There's no way, for example, to allow someone to view the responses but not edit the questions.

## Lack of end-to-end encryption

One final shortcoming with Google Forms: it doesn't support **end-to-end encryption**.
I mentioned before that your data _is_ encrypted **at rest**, so what's the difference?

- **End-to-end encryption** means from the point where someone fills in your form, all the way until the computer you use to view their response, the data is fully encrypted. The form platform you're using cannot see the response data, even if they really wanted to. This is similar to the technology used by many messaging apps such as WhatsApp and Signal.

  This is an extremely strong form of protection since it covers nearly the entire lifecycle of the entered data.

- **Encryption at rest** means the form response is sent to the platform unencrypted (except via the normal HTTPS encryption used by all modern websites), and is then encrypted in the platform's datacentre with the platform's secret key. When you view the response, the platform decrypts it themselves, and sends it to you unencrypted (again, except for HTTPS). The only place the data is encrypted is while it's sitting and waiting on the platform's servers.

  This still adds a good layer of security, but it lacks the full coverage provided by end-to-end encryption, and is therefore often seen as a somewhat less secure option.

Full end-to-end encryption is great for securing the **most crucial form responses**: think sensitive documents and personal information.

It also means that the platform cannot hand over the data to law enforcement authorities or any third-party, even if they wanted to: it's simply **mathematically impossible for them to do so**.
To be clear, there is no evidence to suggest that Google is actively handing out data like this; I'm just saying that there is no cryptography-level constraint that would prvent it.

## Conclusion

Google Forms offers high-quality security guarantees for your data and is an appropriate solution for nearly all survey use cases.
It is fast, reliable, free-of-charge, and very easy to use.

However, there are some easy mistakes that can be made while setting it up, and the lack of end-to-end encryption makes it less reassuring to your users.
There are also numerous other non-security-related reasons to avoid Google Forms, such as its generic and unprofessional appearance, as well as its limited features that have seen a very slow pace of development over the last years.

An excellent alternative is [**Palform**](https://palform.app): like Google Forms, it is **free with unlimited responses** (but with some other limitations). It's much more flexible and has advanced features to support nearly any data collection use case.
Meanwhile, it is still super easy to use and includes **full end-to-end encryption for free**.
Plus, all data is stored safely in the EU.
Join 500+ companies using it and give your users an experience they will love and trust!

---

> Thanks for reading! This article was written by Pal Kerecsenyi, founder of Palform. If you have any questions, please contact mail@support.palform.app.

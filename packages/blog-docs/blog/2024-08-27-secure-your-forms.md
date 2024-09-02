---
authors: [pal]
---

# You should stop using Google Forms

I'm Pal, the founder and developer behind [Palform](https://palform.app), a more secure and full-featured alternative to Google and Microsoft Forms. The idea started when, earlier this year, I noticed a problem with how I collect data when working on projects.

<!-- truncate -->

More often than not, I often found myself in need of a way to collect data from large groups of people related to my work. I needed to create survey-style questionairres, quickly and without too much hassle. Almost every single time, I defaulted to Microsoft or Google forms, like it was muscle memory. These tools were simply _good enough_.

They were both

- easy
- extremely popular
- always free
- just _there_, requiring zero added research into other products
- integrated into whatever ecosystem I was using

In this article, I'll explain why these tools were actually quite problematic, and what I created to try and solve these problems.

## So what's the problem?

Both these tools felt like I was missing something. Many of my end-users made frequent complaints about the privacy records of these companies, and said they felt uncomfortable providing their data through them.

The truth is, Google could see all of your form responses if it wanted to. As a policy, [they don't](https://support.google.com/googlecloud/answer/6056650?sjid=13208649007061461515-EU). But still, I felt weird trusting the world's largest ad-tech corporation with so much data; it's not exactly rare for mega-companies to violate their own terms.

Then there was the question of sovereignty. Google Forms is hosted in the US, EU, and various other places around the world. In the vast majority of non-enterprise use cases (e.g. if you're not [using Google Workspace](https://support.google.com/a/answer/14310028?hl=en) or [your plan isn't good enough](https://workspace.google.com/pricing)), you don't even get a say in _where_ the data is stored. Data transfers between the EU and US (based on the [EU adequacy decision](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/eu-us-data-transfers_en)) require complying with a [complex legal framework](https://www.dataprivacyframework.gov/key-requirements). Since you could be transferring EU user data to the US (or maybe not, you just don't know!), **you have to comply with this framework** if you use Google Forms outside of a Workspace plan that supports limiting your data to a specific region. To me, this was evidently _extremely overkill_ for almost all use-cases.

Of course, there are several excellent [European](https://www.typeform.com/) [companies](https://www.limesurvey.org/) providing form builders. But none of them properly address the growing problem of security. In a data breach, it would still be easily possible for hundreds of thousands of sensitive responses to be leaked. It's a [growing problem in 2024](https://www.idtheftcenter.org/post/itrc-sees-third-most-data-breach-victims-in-quarter/).

## My solution

I wanted to solve this problem, and create a form builder I actually enjoyed using. One that didn't make me and my end-users stress about a random third-party having access to extremely sensitive personal information.

Naturally, I turned to [encryption](https://en.wikipedia.org/wiki/Encryption). What if there was a way to _mathematically_ guarantee the security and privacy of my users?

The design is simple:

- Everyone in your team has a public/private key pair
- When a form is submitted, it's encrypted with the public keys
- Palform stores the encrypted response. I can't see its contents. Nobody can. Not even government authorities.
- Your browser downloads the response, and decrypts it using your private key. You get nice pretty graphs, and full peace of mind!

It's backed by a tried and tested algorithm suite known as [OpenPGP](https://www.openpgp.org/). Millions of emails are regularly sent this way, e.g. using [Proton Mail](https://proton.me/mail). Its security has been backed by decades of research, and [remains unbroken by governments](https://www.openpgp.org/about/history/) (as far as we know).

You can read about all this in more detail [in our documentation](/keys/technical) if you care.

I wanted this to be something everyone could use, so I had to avoid requiring users to understand what all these complex algorithms were. You should be able to go from Google Forms to Palform without even really thinking about it. WhatsApp, Signal, Proton, and many other tools had already mastered the art of providing secure encryption despite many of their users not understanding what _encryption_ means.

**Palform handles all the encryption in the background**. You just have to click a _Generate Key_ button once (when you set up your account), and it's stored nice and safely in your browser. An encrypted backup also gets saved to the server, for which you need to save a key password (similar to the backup key in WhatsApp, for example).

## Making it full-featured

Encryption is cool, but it's not really a selling point unless the form builder itself is just as good as (or preferably better) than competitors. That's why Palform emphasises supporting **loads of question types**.

Some particularly interesting ones are:

- Address auto-complete
- E-signature (I'm considering working on eIDAS compliance)
- Encrypted file upload
- "Hidden" questions that take a value from a URL parameter

I want the company to have a focus on privacy above all else, so I've included **compliance-centric features** such as:
- **submission auto-delete** (after a given number of days)
- OpenID/OAuth so you don't even have to trust me with your credentials
- team/user permission management
- Cloudflare-based privacy-friendly captcha
- and more!

And of course there's dozens of features that make it much more full-featured than Google Forms:

- **Very thorough branding customisation** for a better white-label experience
- Advanced branching rules between question pages
- More specific customisable validation, depending on the question types
- Markdown-enabled description fields
- Automatic correlation analysis between questions in your responses
- Export responses to multiple formats
- [and more coming soon](/roadmap)!

## It wasn't easy
End-to-end encryption really upsets a lot of the things in forms that everyone (including myself) takes for granted.

- When handling **huge amounts of data**, Google Forms can simply generate a pretty overview chart server-side and avoid downloading dozens of megabytes into your browser. Palform cannot, because our server cannot see your data. Instead, we use caching algorithms to progressively stream just the correct minimal responses. We use WASM-based binaries to speed up the process, and depending on your browser, we even support multi-threading. You'll only have to wait a few minutes to decrypt tens of thousands of responses, and from then on it'll load instantly thanks to the cache.
- Some form builders support **webhooks**, where response data is sent to an endpoint of your choice immediately upon a form submission. Palform sends the encrypted data instead. Because OpenPGP keys are so open, you can easily use a server-side key to decrypt the data.
- **Integrations** with third-party apps are basically impossible to offer, unless the app supports decrypting data using a PGP key. I'm trying to solve this by including as many commonly-used 3rd-party features in Palform as possible, but _this is still a work-in-progress_.

## How much should it cost?
Firstly, Palform is _not_ open source at the moment. I understand it's especially important for encryption/security-oriented products to be at least source available, but open sourcing an application hard, and involves a lot of [non-trivial decisions](https://www.theregister.com/2011/06/22/open_source_is_hard/). Palform will be open-sourced at some point in the very near future, most likely under AGPL or a similar license. I might introduce a self-hosted version too, but that's too complicated for now.

Obviously, Google Forms is free. So is Microsoft Forms. But they're not truly "free". Their existence is supported by income from vast advertising networks and shady practices. You're indirectly profiting from _Google's exploitation_ by using Forms.

Okay maybe that's me being a little dramatic. You're allowed to use Google Forms if you want, but please make sure you understand the risks, especially if you're storing something sensitive.

Unfortunately, **Palform needs to cost money**. Hosting infrastructure and spending time working on new features is pricey, and I want it to operate without dodgy VC funding (and definitely without being subsidised by an advertising network).

Some of Palform's [competitors](https://www.typeform.com/pricing/) have unreasonable, arguably _bizarre_ pricing structures. I'm not the only one who feels that way. There's definitely no way the operating costs are quite that high.

I tried to keep it simple and predictable. So here's how I wanted it to work:

- **All plans get unlimited responses**.
  - Responses don't really cost much money to process, since your browser does most of the work.
- There's a free (forever) plan, also with unlimited responses, but with a limited number of forms/questions, and fewer features.
- Higher plans include more features and sometimes higher limits (e.g. for users).
- You always pay a single monthly/yearly price. There's no overages or surprises.
- 2 months free when you pay for a year
- Generous discounts for non-profits, students, and educational organisations
  - As a university student, and a former chair of a non-profit organisation, I know how useful these are!
- I'm friendly; if you want custom pricing for whatever reason, give me a shout!

The detailed pricing I ended up adopting [is available on Palform's home page](https://palform.app/#pricing).

## The future
Palform's the first (of many) side projects that I ended up loving so much that I'm bootstrapping it into a fully-fledged startup. It's been a lot of work, and I've been coding and perfecting it since March 2024. I'm excited to launch it, and really hope you can see its value!

There's many features [planned for the future](/roadmap), and I'm welcoming suggestions. I want to create a form-builder that's browser-first, privacy-first, and as ethically managed as possible. I'm really excited to see many similar tools spring up (e.g. [Plausible](https://plausible.io/)), and I think it's cool to be a part of a European "privacy revolution".

> Palform is developed in the UK and hosted on [Scaleway](https://www.scaleway.com/) servers located exclusively in the EU. It's fully GDPR-compliant and handles minimal data. There's no spyware on our websites.

No matter the size of your business, [encrypting your forms](https://palform.app) and securing their responses has never been more important than today. Not doing so can alienate your users and trigger concerns about your commitment to their privacy. Potentially, it can even create catastrophic data breaches. The good news: it's really easy!

To conclude, please be careful when choosing a form builder. Shiny flashy features are nice, but consider the potential implications of the information you're collecting being leaked or stolen. _Many_ forms collect extremely sensitive personal information, and this needs to be stored responsibly. Palform makes it easy, and even reduces the amount of time you spend complying with the GDPR.

## Try out Palform
You can try it out for **$0/month** with unlimited responses, and upgrade to our low-cost plans only as and when you need.

Get started [on our home page](https://palform.app).

> Thanks for reading! If you have any questions or comments, please email me personally at pal@palform.app.

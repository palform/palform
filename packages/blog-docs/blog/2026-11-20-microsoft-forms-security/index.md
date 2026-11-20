---
authors: [pal]
---

# How secure is Microsoft Forms?

It's an incredibly popular tool, used by organisations across the world for collecting all sorts of data.
But just how secure is it, and can it be trusted with the most sensitive types of information?

Now more than ever, security and privacy are extremely important not only for businesses, but also for consumers,
who increasingly look for clear signs that a company they're dealing with takes steps to protect their data.
Last year alone, [over 375 million people](https://privacyrights.org/resources-tools/reports/2025-data-breach-report) were in at least some way impacted by a data breach, and this number shows no signs of decreasing in the near future.

Making sure you're using the right form tool for the task is crucial.
This article will take a look at the abilities and limitations of Microsoft Forms when it comes to security.

:::note

This article is **written by an actual human**, not an AI model!
I've done my best to check all information as much as possible and provide links as references where I can,
but please let me know if you find any issues.
See the bottom of the article for contact details.

:::

<!-- truncate -->

## How Microsoft Forms actually works

It's not always clear how a seemingly simple tool like Microsoft Forms works behind the scenes, so it's worth having a detailed understanding of it before committing to using it.

- **Each Microsoft Form has a specific owner.** That can be either a user or a group, depending on how you created the form. Subject to certain conditions, you can [change the owner](https://techcommunity.microsoft.com/blog/microsoftformsblog/manage-your-forms-easily-with-ownership-transfer/263230) after you create the form.

- **...but more users can be invited as collaborators.** You can manually add more users or groups, who will be allowed to edit the form and view its responses. It's important to always keep track of who has access, since by default (unless your organisation has implemented restrictions), each individual form can be accessed by a completely different group of people.

- **The questions, design, and settings are all stored on Microsoft's servers.** This data is, like essentially all data in Office 365, [encrypted both in transit and at rest](https://support.microsoft.com/en-us/forms/security-and-privacy-in-microsoft-forms). Every time someone goes to fill in your form, this configuration is served up and rendered in the user's browser, forming a simple cohesive form experience.

- **Form responses go straight to Microsoft's database.** They [previously confirmed](https://techcommunity.microsoft.com/discussions/microsoftforms/microsoft-forms--form-does-not-exist/3876026/replies/3878000) that form data is stored in a dedicated area, separate from OneDrive/SharePoint, except for file uploads and linked Excel workbooks. The responses are also **encrypted in transit and at rest**, but it's crucial to note that this is **not the same as end-to-end encryption**.

- **Customisation is very limited.** Beyond being able to change some design elements, Microsoft Forms doesn't really give you the ability to make the forms your own. Only 7 types of questions are available (8 if you include file uploads, which are only available with certain settings), and few have been added in recent years. While these constraints are annoying, it can be argued that by reducing the complexity of the forms themselves, they're able to focus on making them as secure as possible.

## Microsoft's security measures

As a company focussed on the needs of large enterprises, it goes without saying that **Microsoft has some of the strongest security practices in the industry.**
Here's what they do to keep your data secure:

- They use **encryption in transit and at rest**. Again, this is not the same as end-to-end encryption, and is weaker in a few meaningful ways. It means that your data (i.e. the questions and responses of the form) are sent and received from Microsoft's servers fully encrypted, and once it gets there it's also stored encrypted. This protects against certain types of interception and helps keep your data secure. It's a **fairly industry-standard measure** though; it would be highly concerning if any modern service provider did _not_ offer such encryption.

- You can set up **access controls** to restrict who can fill in your form. While this is more on you as a form owner, it still forms a key security measure: you can set it so that only certain people in your organisation can respond, and you can even log users' Microsoft account details along with their response, giving you a neat and secure integration with the wider Microsoft ecosystem.

- Microsoft as a company implements **wide-ranging general security measures**, not specific to Forms, ranging from physical access controls on their servers to dedicated security teams constantly monitoring their infrastructure. They endorse a [Zero Trust](https://learn.microsoft.com/en-us/microsoft-365/enterprise/essentials-security?view=o365-worldwide) approach, giving only the exact levels of access needed to only the exact people who need them. Additionally, they also have a [bug bounty](https://www.microsoft.com/en-us/msrc/bounty) program, encouraging third-party researchers to actively search for vulnerabilities.

- Forms meets many major **compliance standards**. Microsoft [states](https://support.microsoft.com/en-us/forms/security-and-privacy-in-microsoft-forms) that it complies with GDPR and FERPA, and Forms is [in scope](https://learn.microsoft.com/en-us/compliance/regulatory/offering-hipaa-hitech) for Microsoft's HIPAA Business Associate Agreement, so US healthcare organisations can use it to collect health information, depending on the type of the account.

Overall, it's clear that Microsoft invests heavily in security. After all, it's crucial for a company in their position to earn substantial public trust, and making even a small slip-up could be catastrophic.

That said, their track record isn't spotless. In 2023, a China-based group known as Storm-0558 obtained a Microsoft signing key and used it to break into the email accounts of around two dozen organisations, including US government agencies. A US government review board [later concluded](https://www.cisa.gov/sites/default/files/2025-03/CSRBReviewOfTheSummer2023MEOIntrusion508.pdf) that the attack "should never have happened", blaming "a cascade of security failures at Microsoft" and calling its security culture "inadequate". Then in early 2024, a Russian state-backed group [got into](https://www.microsoft.com/en-us/security/blog/2024/01/25/midnight-blizzard-guidance-for-responders-on-nation-state-attack/) an old test account that wasn't protected by multi-factor authentication, and used it to read emails belonging to Microsoft's senior leadership.

Neither incident involved Microsoft Forms, and Microsoft has since committed to a company-wide security overhaul.
But they're a reminder that even the largest providers make mistakes, and that when they do, any data the provider can read is potentially exposed.

In general, you can rest assured that your respondents' data is in safe hands. However, read on to find out why there are some nuances to this viewpoint.

## The weak spots

Nothing is perfect, and Microsoft Forms is no exception. There are some genuine shortcomings to consider:

- They **don't offer end-to-end encryption**. As mentioned, Microsoft Forms only has a weaker form of encryption which is not fully "end-to-end". True E2EE means that the form response leaves the user's browser encrypted, stays encrypted while being stored on the servers, and then only gets decrypted in the form owner's (your) browser. That way, nobody in between would be able to see the response data, even if they or a hacker tried to. This is the **gold standard** when it comes to data encryption, and is widely used by tools such as [Signal](https://signal.org/) and [Proton Mail](https://proton.me/). 

  - Instead, Microsoft's approach to encryption technically allows their employees to **view your responses** while they're stored on their servers. To be clear, there is nothing to suggest that this is happening, but the point is that it is physically possible for them to do so.

  - In addition, this means that in some cases they could be **legally compelled to hand over response data to authorities**, because they hold it in readable form. Again, this doesn't mean it actually happens on a regular basis, but in the first half of 2026, [they received **29,466** requests](https://www.microsoft.com/en-us/corporate-responsibility/reports/government-requests/customer-data) for information just for consumer (i.e. non-enterprise) data across all of their services, of which around **5%** resulted in them disclosing information.

- The **permissions system can get confusing.** Not only can your form be owned by a user or a group, it can also be shared with multiple users and groups, even ones outside your organisation (depending on your admin's settings). Even worse, if you set up an Excel sheet for the form to sync to, its permissions are managed separately! So if you're not paying close attention, a different set of people might have access to the form itself versus the sheet it's synced to.

- It's not always easy to **customise where your data lives.** Where your data gets stored depends on a few things. 

  - If you're a personal user (i.e. you don't have an enterprise Microsoft 365 subscription), you have essentially no control over where your data lives. It could be in the US or EU, but Microsoft doesn't appear to publish an exact commitment. This isn't always a problem: Microsoft participates in the EU-US Data Privacy Framework, allowing data to (mostly) flow freely between the two areas. But it's still something worth keeping in mind, especially if your form respondents care strongly about the location of their data.
  
  - If you're an enterprise user, where your data lives [depends on where your tenant was created](https://learn.microsoft.com/en-us/microsoft-365/enterprise/m365-dr-service-other?view=o365-worldwide). There's no further way to customise this, for example by selecting an exact country for where your data should be stored. Again, whether this is a problem really depends on your company's needs and your end-users' expectations.

## Consider using end-to-end encrypted forms

To conclude, while Microsoft Forms has **strong overall security measures**, end-to-end encryption is one of the main areas where they fall short. It's definitely worth considering alternatives that bridge this gap, since doing so can **create higher levels of customer trust** and **reduce your exposure to data breaches**.
E2EE does mean that you yourself have to hold the encryption keys, and losing them could mean your data becomes unrecoverable, but many modern tools provide backup options that make this a remote possibility.

One of the main options is [Palform](https://palform.app), an open-source and fully end-to-end encrypted form builder. While it's not part of the Microsoft ecosystem, it supports many of the same features as Microsoft Forms and has a similar user-friendly interface that **doesn't need any technical knowledge**.
It also comes with countless **additional features** on top, like more advanced branching rules, support for custom subdomains, e-signatures, address search fields, and more.
And you can rest assured that all your data is stored in **Germany in the EU** on secure servers.

Plus, it's completely **free** to [get started](https://dash.palform.app/auth/signup) with **unlimited responses included forever**. Join over 1,000 organisations already using it and create your first form in just 5 minutes.

---

> Thanks for reading! This article was written by Pal Kerecsenyi, founder of Palform. If you have any questions, please contact mail@support.palform.app.

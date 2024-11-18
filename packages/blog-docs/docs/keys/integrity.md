---
sidebar_position: 3
---

# Integrity verification

While encryption provides a strong way to prevent data breaches and ensure zero-knowledge storage, a potential trust-related problem can occur.

When choosing the keys to encrypt a given form response with, clients request a list of the form's team's members' keys from the server. In doing so, the client is relying on the server's response being true and trustworthy.

A _compromised_ server could, for example, slip in an additional fake key owned by the attacker, allowing them to circumvent the encryption system for new responses.

In order to combat this issue, Palform implements **key integrity verification**. The system has a few caveats, which this article aims to explain.

## How it works

When you copy the link of a newly created Share Token, a list of OpenPGP key fingerprints is included in the URL [hash/fragment](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash). These are the fingerprints of all the keys of all the users in the team owning the form. For example, if the form is owned by a team containing 3 users, each of whom have 2 registered keys, 6 fingerprints will be included in the URL.

The list of fingerprints is not determined at the time of _creation_ of the Share Token, but rather when you copy the link. If more keys/users are added after the Token's creation and the URL is re-copied, the new keys will be included in the link.

:::warning
Importantly, this means users/keys added to the team after the link is copied won't be able to see any responses, even new ones. If a new key is added, you'll need to re-copy and re-distribute the link.
:::

When the end-user fills in the form, their browser client will request a list of matching keys for the form from the server. It will filter them to only keep the ones identified by fingerprints in the URL hash. Keys that don't match will simply be discarded, with an error only being thrown if no keys are left over (and therefore there's no way to encrypt and submit the form).

## Scope

All standard Share Token links will contain key fingerprints. You can **manually remove the fingerprints list** at the end of the URL to skip integrity verification, but this is done at your own risk.

**Short-link URLs are not supported**. Fingerprints will not be added to short-link URLs (to keep them short), meaning key integrity checking will be skipped.

## Trusting the link

When the URL of a Share Token is generated, the server still provides the list of fingerprints that should be included in it. However, this is not as much of a compromise risk: you can easily manually verify that only the expected certificates have been included.

Simply inspect the URL: look for the part after the `#` near the end:

```
https://dash.palform.app/fill/org_XXX/form_XXX?f=fat_XXX#ak=<key_fingerprints_go_here>
```

As an organisation admin, head to **Organisation** > **Manage keys**. Compare the list of fingerprints to the ones contained in the URL. As long as you don't see any unexpected value, the link is secure and you're guaranteed protection from server compromise.

:::info
See some weird fingerprints that shouldn't be there? Please [contact us](/#speak-to-a-human) immediately.
:::

---
sidebar_position: 2
---

# Multiple keys
You can have as many keys as you want, and this can be useful for some cases.

New form responses will be encrypted so that they can be decrypted by _any_ of your **valid, in-date** keys. If you don't have any valid keys when a response is made, you won't be able to decrypt it at all.

:::warning
If nobody in a form's team has any valid keys at all, new responses will be rejected with an error message
:::

## Expired keys
We recommend **not deleting** expired keys. We'll continue to use them to decrypt existing form responses, but new responses won't be encrypted with them. If a key has expired, you'll need to create or import a new one to continue to be able to receive decryptable responses.

---
sidebar_position: 3
---

# Technical details

This page describes the boring bits behind how end-to-end encryption works on Palform. This is useful for technical users who need to understand what exact security guarantees we offer, and where the limitations are.

## High-level overview

- Organisations are made up of teams, which are made up of users.
- Each form belongs to a team, meaning all members of that team need to be able to see responses for that form.
- Each user has their own encryption key(s). So a given form needs to ensure that all its responses can be decrypted using _all_ the encryption keys of _all_ the users in its team.
- An "encryption key" is actually an **OpenPGP certificate with at least an encryption subkey**. Technically, this can use any algorithm and key size. Palform-generated keys have a secure, performant default, but imported keys may be different.
- When a response is made, the client asks the server for a list of keys. The server responds with the public component of all the current (non-expired) keys of all the users in the form's team. These are anonymised.
- The client encrypts the response data such that _any_ of these keys can decrypt it (by adding a message packet for each key). The encrypted response is a standard OpenPGP message, which is sent to the server ascii-armored.
- The server stores the encrypted messages and serves them to admin users as needed. They can then decrypt them into the raw responses using the private key component (stored locally in the browser).

## Secure defaults

Palform-generated keys (made through the dashboard) are always generated as **Curve25519** with **AES256** and **SHA512**. This has proven _significantly_ faster than RSA-based keys, while having no evidence to suggest it is any less secure.

Currently, the default behaviour is to have "no expiration" (the OpenPGP protocol does not support never-expiring keys, so this actually adds an expiration time several hundred years in the future). Given the managed and centralised system for serving keys, this is a reasonable setup that doesn't introduce significant security concerns (and makes it massively more convenient for our users). You can manually add an expiration date to your generated keys when creating them.

## Backups

Keys (i.e. the full keys, including private components) are always backed up to the Palform server. Obviously, storing them in plaintext would defeat the entire purpose of E2EE, so the frontend client encrypts the key using a randomly-generated passphrase made of English words, and then sends the encrypted backup to the server.

This is useful in a number of cases:

- Transferring your key to another device
- Signing into a new device
- Recovering a lost key (e.g. if you accidentally cleared your browser storage)
- General peace of mind

When you recover a key, the frontend downloads the encrypted backup from the server and tries to decrypt it using the passphrase your provide. If it works, we save the key in your browser, and it's ready to decrypt responses again!

We force creating a backup when you generate a new key, as this encourages secure default behaviour and is less confusing for users to understand. However, you're welcome to manually delete the backup if you _really REALLY_ want by clicking "Manage backup" on the key, followed by "Delete backup".

:::warning
If your key is not backed up (or you forget your backup passphrase) and you lose your local copy, the key will be lost _forever_. There is absolutely nothing we or anyone can do to recover it. This can mean you permanently lose form responses.
:::

## Limitations

No system is perfect, and even ours has certain nuanced limitations that are important to consider to avoid overconfidence.

- **Trust**: when filling in forms, clients must trust the list of keys provided by the server. Of course, you'll be able to see the list of keys that a given response was encrypted with, so you can easily notice if there's a key that doesn't belong to one of your teammates.
- **Backend capabilities**: our backend is completely blind to the contents of your responses. This means we're unable to offer certain features that other form builders might have, such as integrations with CRMs. We hope to provide more features within the encrypted bounds of Palform itself, so we can create a more centralised process for our users without having to rely on potentially insecure third-parties.
- **Scalability**: the more users and keys in a team, the longer it will take to encrypt a form responses. However, it will take _loads_ of users in a given team for this delay to become noticeable, so it's unlikely to ever become a genuine problem.

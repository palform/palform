---
sidebar_position: 1
---

# Creating and importing keys

Managing keys from your dashboard is super easy, and requires absolutely no understanding of how they actually work.

:::info
Keys power the end-to-end encryption behind Palform's form response system. You can find out more about how they work in [our simple explainer](./what).
:::

## Generating a key (recommended)

The easiest (and default) way to create a new Palform key is to generate it through our dashboard.

1. Navigate to **My account > Keys**. Your keys are specific to each organisation, so you'll need to have the correct organisation open.
2. Click **Register new**.
3. (optional) Click **Advanced** to customise the expiration date. See [below](#expiration) for more details.
4. Click **Generate my key**. Follow any prompts to enable browser storage.
5. **Save your key password!** If you lose this, your backup might be inaccessible. It will be physically impossible for us to help you recover lost data in such cases.
6. You're done!


### Expiration

By default, your keys never expire ([with a specific caveat](./technical#secure-defaults) that will almost certainly not affect you).

This is secure, as your keys exist in what's known as a "managed" environment. We look after them for you, and can easily revoke them if they become compromised somehow.

However, if you'd like, you can configure an expiration date. This date will be ingrained into the key itself, so clients will always refuse to encrypt new form responses after the expiration date. You will be able to decrypt existing responses, even after the key expires.

## Importing a key (advanced)

In certain cases, it might be useful to import your own OpenPGP key. We'll add your key to our managed environment and serve it up for form responses as normal.

**Prerequisites**:
- Access to your full OpenPGP certificate with its secret material.
- At least one subkey with the _encrypt_ role.
- No passphrase

Our key importer will reject new keys/certs that don't match these requirements.

This guide applies to GnuPG/`gpg`, which is the most commonly used OpenPGP certificate manager. However, you're welcome to import keys from any other tool too!

1. Generate an ascii-armored export of your key (including secret material).
    ```bash
    gpg --export-secret-keys --armor [key ID or email] > my_secret_key.asc
    ```

2. Navigate to **My account > Keys > Import** in the dashboard.
3. Select **Browse** and choose the `my_secret_key.asc` file created in step 1.
4. Click **Import**.
5. **Save your key password**. This will back the private key up to our server in a secure, encrypted form.
6. You're done! You can delete the locally-saved `my_secret_key.asc` file.

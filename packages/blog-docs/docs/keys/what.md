---
sidebar_position: 0
---

# What are keys?

We get it. Keys are confusing.

Keys power the end-to-end encryption behind Palform, helping to keep your responses extremely secure. This means that third-parties (including us) are completely unable to view the response data, even if they tried really hard.

**You don't need to understand how keys work to use Palform.** However, it might be interesting! So we've created this example for people who have no experience with cryptography.

---

Let's say you have a special **key**. This is made up of two components:

- A _public_ component, that anyone is allowed to see.
- A _private_ component, that you need to keep super secret, and only you should ever see it.

What actually is this key? It's just a jumble of random data! Let's say it looks like this:

> **Public**: E21eEeZ3SHF/60FZkrUILg==

> **Private**: Ojxlo+cKdQwCOHdINPivAg==

You distribute your _public_ jumble to everyone (e.g. by making it public on the Internet). And you keep your private jumble completely secret.

Now, your friend Bob wants to send you a message:

> The quick brown fox jumps over the lazy dog.

Bob finds your _public_ key on the Internet where you put it. He **encrypts** his message using your key.

This means he uses a fancy complex algorithm to _enjumble_ his message using your key. The details of this algorithm are _far_ out of scope for this explainer.

_Encryption_ creates a new, weird-looking version of the message. It looks like random data. Here's the data now, following encryption:

> Q65fiw8HgBDbB3pJue9btgZrWqC99eg1EKWJls03dJKb1GC5uALU0K6D+k7mveL32j303dV9/wNKFxOSJFs3tg

This strange message has a unique property though.

If the _same_ algorithm is applied to this message but using the _private_ key, you'll get the original message back!

Crucially, only the private key can get the original message back. No other key (including the public key) will succeed.

This means Bob can safely send the encrypted message over the Internet, safe in the knowledge that only _you_ can decrypt it, since he's confident you've kept your private key super secret.

---

Palform applies this seemingly simple-sounding concept to form responses.

When you set up your account, **we generate a public/private key pair** for you. The private key stays safely in your browser, and your public key gets sent to our server.

When someone fills in your form, our server sends your public key to their browser. The response gets encrypted, and then sent back to the server. When you view the responses, we send your browser all these encrypted response messages, and your browser decrypts them using your stored private key.

Hopefully that makes sense! We're happy to clear up any questions; just [send us an email](/).

---
authors: [pal]
---

# Palform is now open source!

> Go [check it out on GitHub](https://github.com/palform/palform/)! And maybe give me a :star: if you feel like it

I strongly believe all software with even the slightest security requirement should be open source. Instead of [security through obscurity](https://en.wikipedia.org/wiki/Security_through_obscurity), keeping the code open will help _reduce_ vulnerabilities. The entire community will be easily able to find mistakes and bugs and get them fixed way faster.

<!-- truncate -->

When I started working on Palform, it was a fun side project to improve the way I collected data. Mostly just to fix my personal problems with all the other form builders out there. Deep down, I always intended for it to be open source. Especially since the client-side performs a lot of crucial encryption work, it's essential to be able to see exactly what's going on.

I lovingly use many other open source SaaS tools such as the [Proton](https://proton.me/) suite, [Plausible Analytics](https://plausible.io), [Keycloak](https://www.keycloak.org/), etc. I think the "open source but supported by an _official_ paid SaaS offering" is one of the most sustainable way to do open source projects (besides very large ones such as Linux, which already have a huge amount of volunteers). Otherwise, burnout sets in eventually, because developer time is never truly _free_. Of course, there are exceptions to this, but in general I feel this is a more stable model, despite the possible conflicts of interest that occasionally arise.

My aim with Palform is to ensure everyone can comfortably trust it with very sensitive categories of personal data, the kind you just wouldn't submit in a regular Google Form. This will hopefully move the project closer to building that trust.

## Licensing

It's always a big question.

For now, I've opted for the [GNU AGPL](https://www.gnu.org/licenses/agpl-3.0.en.html). Perhaps my inner fears of some evil megacorporation taking my code to build a data-stealing version of Palform are what kept me from going for something simpler like the MIT license. Realistically, that's unlikely to happen, but you never know...

In any case, I don't think there _should_ be a situation where a modification to my source code on a cloud-hosted service shouldn't be made public. That would just inherently be dodgy.

## Contributions

...are welcome! Of course, it'll take some time until I can document everything about the codebase to a point where it's easy and accessible to contributors. But if you feel like browsing through some Rust code to fix any bugs, I would love that too!

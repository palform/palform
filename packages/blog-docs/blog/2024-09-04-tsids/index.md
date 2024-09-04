---
authors: [pal]
---

# How type-safe scalable database IDs reduce bugs at Palform

![A datacentre with a row of disks. Green lights flashing.](./banner.jpg)

Database IDs are one of those things you really don't want to think about. It feels like they should just "work". Why are there different types of IDs?? Why can't I just use an auto-incrementing number? The real world is just so annoying.

In this article, I'll explain why UUIDs look like the cool solution to all our problems (but actually aren't), and demonstrate a simple but powerful system I used at Palform to vastly improve the database experience.

<!-- truncate-->

## The problems with UUIDs

Nowadays, UUIDs are often the go-to for the majority of developers. Essentially all major databases, ORMs, clients, and SDKs have built-in or high-quality 3rd-party support for them.

Yes they're slightly more difficult to generate, but you don't have to write code for that anymore!

With 128 bits of data, it's practically impossible to ever get a collision so you're safe from that too.

Around the internet, there are [several](https://blog.boot.dev/clean-code/what-are-uuids-and-should-you-use-them/) [blog](https://www.howtogeek.com/devops/what-are-uuids-and-why-are-they-useful/) [posts](https://www.uniqueids.org/en/what-is-uuid) advocating for their use over sequential IDs. And it's true: it's probably nearly always the better choice between the two. It hides the number of records you have and prevents automated enumeration, while not impacting performance _too_ much in smaller databases.

But with [Palform](https://palform.app), my aim was to handle hundreds of thousands of form responses smoothly and lag-free. "Good enough" won't do, so we need a better solution.

## Enter TSIDs

Already gaining traction between developers, TSIDs [strike a balance](https://www.foxhound.systems/blog/time-sorted-unique-identifiers/) between incremental IDs and UUIDs. They're more space-efficient than the latter, while having the same time-sortable property of the former (although it's not fully guaranteed).

I won't write about the details of how they work here, as other posts have already done a much better job than I could.

They can be a bit of a pain with distributed systems as they require a node ID, which is not always easy to obtain in a definitely unique way. However, Palform is not really a distributed system.

Support is also much weaker than UUIDs, but that's also fine: luckily, Rust has an [excellent crate for it](https://crates.io/crates/tsid) which is really all I needed.

UUIDs are serialised in a [highly standardised way](https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-format). They're so widespread that most developers can glance at the random-looking data and immediately tell it's a UUID. Just look at it:

```
de6fc6ac-2c80-48ce-9b48-f9793a0b8c71
```

Mmmm beautiful.

TSIDs are 64 bit integers. They're usually serialised into a 13 character base-32 URL-safe string. That's much shorter and arguably more readable than a UUID:

```
0GYDWW6VM4C0E
```

Even nicer.

So I made the decision: Palform would use TSIDs. But how to implement them?

## SeaORM support

Palform uses the excellent [SeaORM](https://www.sea-ql.org/SeaORM/) to manage database records on the backend. It has support for Postgres' built-in `uuid` type, but understandably no support for the relatively new TSID.

SeaORM requires declaring Rust entities (basically a struct) for each of your tables. This then makes it easy to write very Rust-idiomatic and satisfying database queries. Here's what an entity looks like when using UUIDs:

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_token")]
pub struct Model {
    pub hash: String,
    pub created_at: DateTime,
    pub expires_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
}
```

This uses the very popular [`uuid`](https://crates.io/crates/uuid) crate, and SeaORM takes care of converting to/from the PostgreSQL type.

Creating a new record is very easy:

```rust
let new_token = admin_user::ActiveModel {
    id: Set(Uuid::new_v4()),
    user_id: Set(user_id),
    // ...
    ..Default::default()
};

new_user.insert(conn).await?;
```

Now how to do this with TSIDs? The `tsid` create contains a `create_tsid` function, which returns a `TSID` struct. We can turn that into a `u64`, too:

```rust
{
    id: Set(create_tsid().number()),
    // ...
}
```

We also need a way to convert back from `u64` to the `TSID` struct. It's all a bit awkward, and the typing is weird: what's a `u64`? It could be any number, how do we guarantee it's a TSID?

## Building my own adapter

It felt like an upward battle at this point, and I kept thinking how much easier it would be to just settle with the beautifully supported UUIDs. But this was getting interesting now, and so I decided to spend nearly an entire week designing a custom adapter. I had a plan.

I wanted my IDs to be _type-safe_. That is, each table in my database should somehow have its own type of ID, and Rust should complain violently if, for example, I try to use a User ID in a place where a Question ID is needed.

First, I'd need a struct that I can base everything off. This would still involve the `tsid` crate, just with a lot of wrapping code. Here's how I started:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PalformDatabaseID {
    pub(crate) id: tsid::TSID,
}
```

Next came the typing. Stripe, famous for its much-loved developer API, does something similar by [prefixing IDs](https://dev.to/stripe/designing-apis-for-humans-object-ids-3o5a) with a short code that identifies the resource. For example, customer IDs are prefixed with `cus_`. Not only does it avoid confusion in code, it's also super readable, as you can tell what an ID is referring to without any additional information.

Inspired by this very satisfying system, I wanted to create something similar.

I created a trait:

```rust
pub trait PalformIDResource: Eq + PartialEq + Clone + Copy + Hash + Send {
    fn prefix() -> Option<String>;
}
```

The prefix is an `Option` to support certain edge cases where I need to be able to refer to _any_ type of ID in a single field (e.g. audit logs). Where the prefix is unknown, I use `None`.

Next, I simply created a type implementing this trait for each of my tables:

```rust
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct IDOrganisation;
impl PalformIDResource for IDOrganisation {
    fn prefix() -> Option<String> {
        Some("org".to_owned())
    }
}
```

With some macros added in for better readability, you can see [all the resource declarations on GitHub](https://github.com/palform/palform/blob/main/packages/tsid/src/resources.rs) (give me a star while you're there :heart:).

Then, I modified the `PalformDatabaseID` struct to include the prefix.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PalformDatabaseID<Resource: PalformIDResource> {
    pub(crate) id: TSID,
    resource: PhantomData<Resource>,
}
```

Then came the laborious part: adding all the integrations needed for this to work with Serde, SeaORM, Rocket, the OpenAPI generator, etc. You can see [everything on GitHub](https://github.com/palform/palform/tree/main/packages/tsid/src).

## Rewriting the entities
Now, I could modify the SeaORM entities to use my shiny adapter.

```rust
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDAuthToken>,
}
```

A `PalformDatabaseID<IDAuthToken>` is completely distinct from a `PalformDatabaseID<IDOrganisation>`, for example. Rust will fully block the compilation if it's somehow mismatched, which is exactly what I wanted.

Generating them is super easy:
```rust
let new_form = form::ActiveModel {
    id: Set(PalformDatabaseID::<IDForm>::random()),
    // ...
}
```

In the database, it's still just stored as a type-independent `u64`. But whenever it's handed to the application, it has to choose a typed prefix, which is also conveyed in the serialised form (e.g. `org_0GYDWW6VM4C0E`). When being deserialised, the prefix is checked to ensure it matches the requested type; if it's wrong (or there is no prefix), a scary error is returned.

This actually helped catch some errors! In several places, I was using the UUID of the wrong resource without noticing; everything was just `Uuid` before, and I had to rely on variable names to keep track of what was what.

For future development, it will also inevitably reduce bugs and small but high-consequence typos. It will likely even improve the security posture of Palform, which is absolutely essential with such a security-focussed application. Developers are only humans and even the most careful ones will make mistakes; this change, however, will stop the mistakes with big scary red error messages before they can cause any damage.

It's also simply more Rust-like, which I'm sure will make the Rust people very happy.

## Sharing the code

Palform is [open source in its entirety](https://github.com/palform/palform/), currently under the AGPL license. The ID management is under the `packages/tsid` directory, but I might release it as a more liberally licensed separate module in the near future, so that everyone building web apps in Rust can benefit from these performance, security, and stability improvements.

In the meantime, if you're ever in need of a form builder that actually cares about encryption, privacy, scalability, and other ethical principles, give [Palform](https://palform.app) a try (it's free with unlimited responses).

> Thanks for reading! This article was written by [Pal Kerecsenyi](https://www.linkedin.com/in/palkerecs), the founder of Palform. Last updated 04/09/2024.

> If you have any questions or found a mistake, please email me at pal@palform.app.

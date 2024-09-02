# [Palform](https://palform.app)

Palform is a form builder that's:

- open source
- end-to-end encrypted
- privacy focussed (minimal third party scripts)
- full featured
- free to use [online](https://palform.app) on our European-hosted secure servers

Our _entire_ codebase is open source, including our marketing pages. This repository contains everything needed to operate the service.

## Architecture
Palform is made up of several Rust and JS components.

We use Rust with the [Rocket](https://github.com/rwf2/Rocket) framework and [SeaORM](https://github.com/SeaQL/sea-orm) on the backend. This helps maintain very high performance; the observed memory and CPU consumption in practice has been practically zero. Although implementing features is arguably more difficult, the stability guarantees are important and useful.

Database IDs are in the form of a custom TSID, with each resource type having its own prefix. Each prefix is mapped to a Rust type, reducing the risk of providing the ID of the wrong kind of resource in code.

The main frontend app is written in Svelte. OpenAPI bindings in Typescript are automatically generated using the `make frontend_openapi` command.

The `analysis` and `client-common` components are also written in Rust, containing components that are either exclusively for frontend use, or for shared use between the frontend and backend. On the frontend, these components are compiled into WASM binaries and accompanying bindings using [wasm-pack](https://github.com/rustwasm/wasm-pack). Some things like encryption and bulk analytics needs really good performance, which we've found to be massively better with WASM as opposed to native JS libraries.

We use the [Sequoia PGP](https://gitlab.com/sequoia-pgp/sequoia/-/tree/main/openpgp) library for handling form encryption and key management on both the backend and frontend.

We use [Astro](https://astro.build/) and Svelte for the landing page.

We use [Docusaurus](https://docusaurus.io/) for the blog and documentation pages.

## Contributing
We welcome contributions! Please submit a pull request with any changes, and we'll review them. Please note, we might reject some features that don't align with our mission. We want Palform to be full featured, but to avoid "feature creep" by introducing things that are far away from the original idea.

The copyright of any code you submit will belong to Palform Ltd, a registered company in England/UK (15796859). By submitting any code contributions, you accept this.

## Self hosting
This repository does not operate any versioned releases, so the code in the `main` branch cannot be assumed to be stable at any point. We'll implement a formal versioning system in the future.

Currently, self hosting is possible but not officially supported. We don't have any documentation on how to do it yet, but we're still working it out.

We cannot assume any liability for things that might go wrong on a self hosted instance.

## License
The source code is provided under an AGPL license. Please see LICENSE.md.

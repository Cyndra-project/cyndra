<!-- markdownlint-disable -->
<p align="center">
<img width="300" src="https://raw.githubusercontent.com/cyndra-hq/cyndra/master/assets/logo-rectangle-transparent.png"/>
</p>
<br>
<p align="center">
  <a href="https://github.com/cyndra-hq/cyndra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://docs.cyndra.rs/">
    <img alt="docs" src="https://img.shields.io/badge/docs-cyndra.rs-orange">
  </a>
  <a href="https://docs.rs/cyndra-runtime">
    <img alt="crate-docs" src="https://img.shields.io/badge/docs-docs.rs-orange">
  </a>
  <a href="https://status.cyndra.rs/">
    <img alt="status" src="https://img.shields.io/badge/status-blue">
  </a>
  <a href="https://circleci.com/gh/cyndra-hq/cyndra/">
    <img alt="build status" src="https://circleci.com/gh/cyndra-hq/cyndra.svg?style=shield"/>
  </a>
</p>
<p align="center">
  <a href="https://crates.io/crates/cargo-cyndra">
    <img alt="crates" src="https://img.shields.io/crates/d/cargo-cyndra">
  </a>
  <a href="https://discord.gg/cyndra">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
  <a href="https://twitter.com/cyndra_dev">
    <img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/cyndra_dev">
  </a>
</p>
<p align="center">
  <a href="https://console.algora.io/org/cyndra/bounties?status=open">
    <img alt="open bounties" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fconsole.algora.io%2Fapi%2Fshields%2Fcyndra%2Fbounties%3Fstatus%3Dopen"/>
  </a>
  <a href="https://console.algora.io/org/cyndra/bounties?status=completed">
    <img alt="rewarded bounties" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fconsole.algora.io%2Fapi%2Fshields%2Fcyndra%2Fbounties%3Fstatus%3Dcompleted"/>
  </a>
</p>
<p align="center">
  🎄 <b>Cyndra Christmas Code Hunt</b> is here! Take on 16 days of Rust challenges and win prizes! <a href="https://www.cyndra.rs/cch">Click here to sign up!</a> 🎁
</p>
<!-- markdownlint-restore -->

---

# Cyndra

[Cyndra](https://www.cyndra.rs/) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

Cyndra is built for productivity, reliability and performance:

- Zero-Configuration support for Rust using annotations
- Automatic resource provisioning (databases, caches, subdomains, etc.) via [Infrastructure-From-Code](https://www.cyndra.rs/blog/2022/05/09/ifc)
- First-class support for popular Rust frameworks ([Actix Web](https://docs.cyndra.rs/examples/actix), [Rocket](https://docs.cyndra.rs/examples/rocket), [Axum](https://docs.cyndra.rs/examples/axum), and [more](https://docs.cyndra.rs/examples/other))
- Support for deploying Discord bots using [Serenity](https://docs.cyndra.rs/examples/serenity)
- Scalable hosting (with optional self-hosting in the future)

📖 Check out our documentation to get started quickly: [docs.cyndra.rs](https://docs.cyndra.rs)

🙋‍♂️ If you have any questions, join our [Discord](https://discord.gg/cyndra) server.

⭐ If you find Cyndra interesting, and would like to stay up-to-date, consider starring this repo to help spread the word.

![star](https://i.imgur.com/kLWmThm.gif)

## (NEW) Cyndra Console

Your projects can now be viewed on the brand new [Cyndra Console](https://console.cyndra.rs/)!
The CLI is still used for most tasks.

![console-preview](https://i.imgur.com/1qdWipP.gif)
*The GIF above visualizes the ease of adding resources to your project(s), along with how they are displayed in the console.*

## Getting Started

The `cargo-cyndra` CLI can be installed with a pre-built binary or from source with cargo.

Cyndra provides pre-built binaries of the `cargo-cyndra` CLI with every release
for most platforms, they can be found on [our GitHub](https://github.com/cyndra-hq/cyndra/releases/latest).

On Linux and macOS, you can use this install script, which will automatically install the correct target for your OS and distro:

```sh
curl -sSfL https://www.cyndra.rs/install | bash
```

Our binaries can also be installed using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall).
To install with `cargo-binstall`, run:

```sh
# cargo-binstall can also be installed directly as a binary to skip the compilation time: https://github.com/cargo-bins/cargo-binstall#installation
cargo install cargo-binstall
cargo binstall cargo-cyndra
```

Although a bit slower, you can also install directly with cargo:

```sh
cargo install cargo-cyndra
```

> If installing binstall or cargo-cyndra fails, try adding `--locked` to the install command

After installing, log in with:

```sh
cargo cyndra login
```

To initialize your project, simply write:

```bash
cargo cyndra init --template axum hello-world
# Choose a unique project name!
```

And to deploy it, write:

```bash
cd hello-world
cargo cyndra project start  # Only needed if project has not already been created during init
cargo cyndra deploy --allow-dirty
```

And... that's it!

```text
Service Name:  hello-world
Deployment ID: 3d08ac34-ad63-41c1-836b-99afdc90af9f
Status:        running
Last Updated:  2022-04-01T08:32:34Z
URI:           https://hello-world.cyndraapp.rs
```

Feel free to build on top of the generated `hello-world` boilerplate or take a stab at one of our [examples](https://github.com/cyndra-hq/cyndra-examples).

For the full documentation, visit [our docs](https://docs.cyndra.rs).

## Repositories

| Name | Description |  |  |
|-|-|-|-|
| [cyndra](https://github.com/cyndra-hq/cyndra) 🚀 (This repo) | The core Cyndra product. Contains all crates that users interact with. | [Issues](https://github.com/cyndra-hq/cyndra/issues) | [PRs](https://github.com/cyndra-hq/cyndra/pulls)
| [cyndra-examples](https://github.com/cyndra-hq/cyndra-examples) 👨‍🏫 | Officially maintained examples of projects that can be deployed on Cyndra. Also has a list of [community examples](https://github.com/cyndra-hq/cyndra-examples#community-examples). | [Issues](https://github.com/cyndra-hq/cyndra-examples/issues) | [PRs](https://github.com/cyndra-hq/cyndra-examples/pulls)
| [cyndra-docs](https://github.com/cyndra-hq/cyndra-docs) 📃 | Documentation hosted on [docs.cyndra.rs](https://docs.cyndra.rs/). | [Issues](https://github.com/cyndra-hq/cyndra-docs/issues) | [PRs](https://github.com/cyndra-hq/cyndra-docs/pulls)
| [www](https://github.com/cyndra-hq/www) 🌍 | Our website [cyndra.rs](https://www.cyndra.rs/), including the [blog](https://www.cyndra.rs/blog/tags/all) and [Launchpad newsletter](https://www.cyndra.rs/launchpad). | [Issues](https://github.com/cyndra-hq/www/issues) | [PRs](https://github.com/cyndra-hq/www/pulls)
| [deploy-action](https://github.com/cyndra-hq/deploy-action) ⚙ | GitHub Action for continuous deployments. | [Issues](https://github.com/cyndra-hq/deploy-action/issues) | [PRs](https://github.com/cyndra-hq/deploy-action/pulls)
| [awesome-cyndra](https://github.com/cyndra-hq/awesome-cyndra) 🌟 | An awesome list of Cyndra-hosted projects and resources that users can add to. | [Issues](https://github.com/cyndra-hq/awesome-cyndra/issues) | [PRs](https://github.com/cyndra-hq/awesome-cyndra/pulls)

## Contributing to Cyndra

Contributing to Cyndra is highly encouraged!

Check out our [contributing docs](./CONTRIBUTING.md) and find the appropriate repo above to contribute to.

For development of this repo, check the [development docs](./DEVELOPING.md).

Even if you are not planning to submit any code, joining our [Discord server](https://discord.gg/cyndra) and providing feedback helps us a lot!

### Algora Bounties 💰

To offload work from the engineering team on low-priority issues, we will sometimes add a cash bounty to issues.
Sign up to the [Algora Console](https://console.algora.io/org/cyndra/bounties?status=open) to find open issues with bounties.

## Community and Support

- [GitHub Issues](https://github.com/cyndra-hq/cyndra/issues). Best for: bugs and errors you encounter using Cyndra.
- [Twitter](https://twitter.com/cyndra_dev). Best for: keeping up with announcements, releases, collaborations and other events.
- [Discord](https://discord.gg/cyndra). Best for: *ALL OF THE ABOVE* + help, support, sharing your applications and hanging out with the community.

## Project Status

Check for any outages and incidents on [Cyndra Status](https://status.cyndra.rs/).

We are currently in Public Beta.
Watch "releases" of this repo to get notified of major updates!
Also, check out the [Beta announcement](https://www.cyndra.rs/beta#06) for features we are looking forward to.

- [x] Alpha: We are testing Cyndra, API and deployments may be unstable
- [x] Public Alpha: Anyone can sign up, but go easy on us,
  there are a few kinks
- [x] Public Beta: Stable enough for most non-enterprise use-cases
- [ ] Public: Production-ready!

## Contributors ✨

Thanks goes to these wonderful people:

<!-- markdownlint-disable -->
<a href="https://github.com/cyndra-hq/cyndra/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyndra-hq/cyndra" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

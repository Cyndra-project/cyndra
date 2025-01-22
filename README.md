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
<br>

<h1 align="center">Fastest Way to Build & Ship Rust Apps</h1>
<div align="center">
Get resources and deploy your apps with a few lines of code.
</div>

<h3 align="center">Simple. &nbsp; Easy. &nbsp; Joyful.</h3>

<p align="center">
    <a href="https://github.com/cyndra-hq/cyndra/issues/new?assignees=&labels=S-Triage%2CT-Bug&projects=&template=BUG-REPORT.yml&title=%5BBug%5D%3A+">Report Bug</a>
    ·
    <a href="https://github.com/cyndra-hq/cyndra/issues/new?assignees=&labels=S-Triage%2CT-Feature+Request&projects=&template=FEATURE-SUGGESTION.yml&title=%5BFeature%5D%3A+">Request a Feature</a>
    ·
  <a href="https://discord.gg/cyndra">Join Our Discord</a>
    ·
    <a href="https://twitter.com/cyndra_dev">Follow us on Twitter</a>
  </p>

<div align="center"><img src="https://i.imgur.com/1qdWipP.gif" width="100%" ></div>
<br>


*<div align="center">⭐ If you find Cyndra interesting, consider starring this repo to help spread the word.</div>*
<br>

# Features

* __One-line Resource Provisioning:__ Get a database, or any other AWS resource by adding a line of code to your main file. To delete one, just remove that line of code. No config/yaml files required.
* __Rapid Development:__ It takes 2 minutes from project initialization to a deployed project. It takes another 2 minutes to provision a resource, and get it deployed to production.
* __First-class support for popular Rust frameworks:__ [Axum](https://docs.cyndra.rs/examples/axum), [Actix Web](https://docs.cyndra.rs/examples/actix), [Rocket](https://docs.cyndra.rs/examples/rocket), and [more](https://docs.cyndra.rs/examples/other)
*  __Security:__ Let us worry about the security & permissions while you focus on writing good code.
<br>
<br>

# Quick Start

On Linux and macOS, you can use this install script, which will automatically install the correct target for your OS and distro:

```sh
curl -sSfL https://www.cyndra.rs/install | bash
```

On Windows, you can use this install script to do the same:

```powershell
iwr "https://www.cyndra.rs/install-win" | iex
```

After installing, log in with:

```sh
cargo cyndra login
```

To initialize your project, simply write:

```bash
cargo cyndra init --template axum hello-world
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

# Quick Look

Below is a basic "Hello World" application written in Axum:
```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
```

In order to be able to deploy it with a single command, we update the snippet as follows:

```rust
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

Now, with just `cargo cyndra deploy`, you can see your application live. But let's enhance it further by adding a shared Postgres database:

```rust
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn main(
    #[cyndra_shared_db::Postgres] pool: sqlx::PgPool,
) -> cyndra_axum::CyndraAxum {

    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("failed to run migrations");

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

Now, if we run `cargo cyndra deploy`, we'll have an up and running project with a database inside & ready to use.

# Repositories

| Name | Description |
|-|-|
| [cyndra](https://github.com/cyndra-hq/cyndra) 🚀 (This repo) | The core Cyndra product. Contains all crates that users interact with. |
| [cyndra-examples](https://github.com/cyndra-hq/cyndra-examples) 👨‍🏫 | Officially maintained examples of projects that can be deployed on Cyndra. Also has a list of [community examples](https://github.com/cyndra-hq/cyndra-examples#community-examples). |
| [cyndra-docs](https://github.com/cyndra-hq/cyndra-docs) 📃 | Documentation hosted on [docs.cyndra.rs](https://docs.cyndra.rs/). |
| [www](https://github.com/cyndra-hq/www) 🌍 | Our website [cyndra.rs](https://www.cyndra.rs/), including the [blog](https://www.cyndra.rs/blog/tags/all) and [Launchpad newsletter](https://www.cyndra.rs/launchpad). |
| [deploy-action](https://github.com/cyndra-hq/deploy-action) ⚙ | GitHub Action for continuous deployments. |
| [awesome-cyndra](https://github.com/cyndra-hq/awesome-cyndra) 🌟 | An awesome list of Cyndra-hosted projects and resources that users can add to. |
| [shuttlings](https://github.com/cyndra-hq/shuttlings) ⚔️ | A collection of Rust code challenges. A great way to get started with using Rust and Cyndra. |

# Contributing to Cyndra

Contributing to Cyndra is highly encouraged!
Even if you are not planning to submit any code, joining our [Discord server](https://discord.gg/cyndra) and providing feedback helps us a lot!

Check out our [contributing docs](./CONTRIBUTING.md) and find the appropriate repo above to contribute to.
For development of this repo, check the [development docs](./DEVELOPING.md).

### Algora Bounties 💰

To offload work from the engineering team on low-priority issues, we will sometimes add a cash bounty to issues.
Sign up to the [Algora Console](https://console.algora.io/org/cyndra/bounties?status=open) to find open issues with bounties.

# Project Status

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

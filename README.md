<p align="center">
<img width="300" src="https://raw.githubusercontent.com/getsynth/cyndra/master/resources/logo-rectangle-transparent.png"/>
</p>
<br>
<p align=center>
  <a href="https://docs.rs/cyndra-service">
    <img alt="docs" src="https://img.shields.io/badge/doc-reference-orange">
  </a>
  <a href="https://github.com/getsynth/cyndra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://github.com/getsynth/cyndra/actions">
    <img alt="build status" src="https://img.shields.io/github/workflow/status/getsynth/cyndra/cargo-test"/>
  </a>
  <a href="https://discord.gg/H33rRDTm3p">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
</p>

---

# cyndra

[Cyndra](https://www.cyndra.rs/) is a serverless platform for Rust which makes it really easy to 
deploy your web-apps.

Cyndra is built for productivity, reliability and performance:
- Zero-Configuration support for Rust using annotations
- Automatic resource provisioning (databases, caches, subdomains, etc.) via [Infrastructure-From-Code](https://www.cyndra.rs/blog/2022/05/09/ifc)
- First-class support for popular Rust frameworks ([Rocket](https://github.com/cyndra-hq/cyndra/tree/main/examples/rocket/hello-world), [Axum](https://github.com/cyndra-hq/cyndra/tree/main/examples/axum/hello-world), 
  [Tide](https://github.com/cyndra-hq/cyndra/tree/main/examples/tide/hello-world) and [Tower](https://github.com/cyndra-hq/cyndra/tree/main/examples/tower/hello-world))
- Scalable hosting (with optional self-hosting)


## Getting Started

First download the Cyndra cargo extension and login:

```bash
$ cargo install cargo-cyndra
$ cargo cyndra login
$ cargo init --lib hello-world
```

Update your `Cargo.toml`:

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[lib]

[dependencies]
rocket = "0.5.0-rc.1"
cyndra-service = { version = "0.3.3", features = ["web-rocket"] }
```


Create your first cyndra app in `lib.rs`:

```rust
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_service::main]
async fn rocket() -> Result<Rocket<Build>,cyndra_service::Error> {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket)
}
```

Deploy:

```bash
$ cargo cyndra deploy
   Finished dev [unoptimized + debuginfo] target(s) in 1m 01s

        Project:            hello-world
        Deployment Id:      3d08ac34-ad63-41c1-836b-99afdc90af9f
        Deployment Status:  DEPLOYED
        Host:               hello-world.cyndraapp.rs
        Created At:         2022-04-01 08:32:34.412602556 UTC
        Database URI:       postgres://***:***@pg.cyndra.rs/db-hello-world
```

For the full documentation, visit [docs.rs/cyndra-service](https://docs.rs/cyndra-service)

## Working on cyndra

If you want to setup a local environment to test code changes to core `cyndra` packages, follow these steps.

Build the required images with 

```bash
$ docker buildx bake -f docker-bake.hcl provisioner backend
```

The images get build with [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) and therefore support incremental builds (most of the time). So they will be much faster to re-build after an incremental change in your code - should you wish to deploy it locally straightaway.

Create a docker persistent volume with

```bash
$ docker volume create cyndra-backend-vol
```

Finally, you can start a local deployment of the backend with

```bash
$ docker compose -f docker-compose.dev.yml up -d
```

The API is now accessible on `localhost:8000` (for app proxies) and `localhost:8001` (for the control plane). When running `cargo run --bin cargo-cyndra` (in a debug build), the CLI will point itself to `localhost` for its API calls. The deployment parameters can be tweaked by changing values in the [.env](./.env) file.

In order to test local changes to the `cyndra-service` crate, you may want to add the following to a `.cargo/config.toml` file:

``` toml
[patch.crates-io]
cyndra-service = { path = "[base]/cyndra/service" }
```

See [Overriding Dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html) for more.

## Roadmap

For a comprehensive view of the cyndra roadmap check out this [project board](https://github.com/orgs/cyndra-hq/projects/4).

If you have any requests or suggestions feel free to open an issue.

## Community & Support

- [Community Forum](https://github.com/getsynth/cyndra/discussions). Best for: help with building, discussion about best practices.
- [GitHub Issues](https://github.com/getsynth/cyndra/issues). Best for: bugs and errors you encounter using Cyndra.
- [Discord](https://discord.gg/H33rRDTm3p). Best for: sharing your applications and hanging out with the community.
- [Twitter](https://twitter.com/cyndra_dev). Best for: keeping up with announcements and releases

## Status

- [x] Alpha: We are testing Cyndra, API and deployments may be unstable
- [x] Public Alpha: Anyone can sign up, but go easy on us, 
  there are a few kinks
- [ ] Public Beta: Stable enough for most non-enterprise use-cases
- [ ] Public: Production-ready!

We are currently in Public Alpha. Watch "releases" of this repo to get 
notified of major updates!

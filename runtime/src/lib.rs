#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/logo-square-transparent.png",
    html_favicon_url = "https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/favicon.ico"
)]
//! # Cyndra - Deploy Rust apps with a single Cargo subcommand
//! <div style="display: flex; margin-top: 30px; margin-bottom: 30px;">
//! <img src="https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/logo-rectangle-transparent.png" width="400px" style="margin-left: auto; margin-right: auto;"/>
//! </div>
//!
//! Hello, and welcome to the <span style="font-family: Sans-Serif;"><a href="https://cyndra.rs">cyndra</a></span> API documentation!
//!
//! Cyndra is an open-source app platform that uses traits and annotations to configure your backend deployments.
//!
//! ## Usage
//! Start by installing the [`cargo cyndra`](https://docs.rs/crate/cargo-cyndra/latest) subcommand by running the following in a terminal:
//!
//! ```bash
//! $ cargo install cargo-cyndra
//! ```
//!
//! Now that cyndra is installed, you can initialize a project with Axum boilerplate:
//! ```bash
//! $ cargo cyndra init --template axum my-axum-app
//! ```
//!
//! By looking at the `Cargo.toml` file of the generated `my-axum-app` project you will see it has been made to
//! be a binary crate with a few dependencies including `cyndra-runtime` and `cyndra-axum`.
//!
//! ```toml
//! cyndra-runtime = "0.24.0"
//! axum = "0.6.10"
//! cyndra-axum = "0.24.0"
//! tokio = "1.26"
//! ```
//!
//! A boilerplate code for your axum project can also be found in `src/main.rs`:
//!
//! ```rust,no_run
//! use axum::{routing::get, Router};
//!
//! async fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn axum() -> cyndra_axum::CyndraAxum {
//!     let router = Router::new().route("/hello", get(hello_world));
//!
//!     Ok(router.into())
//! }
//! ```
//!
//! Check out [our docs](https://docs.cyndra.rs/introduction/welcome) to see all the frameworks we support, or
//! our [examples](https://github.com/cyndra-hq/cyndra-examples) if you prefer that format.
//!
//! ## Running locally
//! To test your app locally before deploying, use:
//!
//! ```bash
//! $ cargo cyndra run
//! ```
//!
//! You should see your app build and start on the default port 8000. You can test this using;
//!
//! ```bash
//! $ curl http://localhost:8000/hello
//!
//! Hello, world!
//! ```
//!
//! ## Deploying
//!
//! You can deploy your service with the [`cargo cyndra`](https://docs.rs/crate/cargo-cyndra/latest) subcommand too.
//! But, you will need to authenticate with the cyndra service first using:
//!
//! ```bash
//! $ cargo cyndra login
//! ```
//!
//! This will open a browser window and prompt you to connect using your GitHub account.
//!
//! Before you can deploy, you have to create a project. This will start a deployer container for your
//! project under the hood, ensuring isolation from other users' projects. PS. you don't have to do this
//! now if you did in in the `cargo cyndra init` flow.
//!
//! ```bash
//! $ cargo cyndra project start
//! ```
//!
//! Then, deploy the service with:
//!
//! ```bash
//! $ cargo cyndra deploy
//! ```
//!
//! Your service will immediately be available at `{crate_name}.cyndraapp.rs`. For example:
//!
//! ```bash
//! $ curl https://my-axum-app.cyndraapp.rs/hello
//! Hello, world!
//! ```
//!
//! ## Using `sqlx`
//!
//! Here is a quick example to deploy a rocket service that uses a postgres database and [sqlx](http://docs.rs/sqlx):
//!
//! Initialize a project with Rocket boilerplate:
//! ```bash
//! $ cargo cyndra init --template rocket my-rocket-app
//! ```
//!
//! Add `cyndra-shared-db` as a dependency with the `postgres` feature, and add `sqlx` as a dependency with the
//! `runtime-tokio-native-tls` and `postgres` features inside `Cargo.toml`:
//!
//! ```toml
//! cyndra-shared-db = { version = "0.24.0", features = ["postgres"] }
//! sqlx = { version = "0.7.1", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
//!
//! Now update the `#[cyndra_runtime::main]` function to take in a `PgPool`:
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! use rocket::State;
//! use sqlx::PgPool;
//! use cyndra_rocket::CyndraRocket;
//!
//! struct MyState(PgPool);
//!
//! #[get("/hello")]
//! fn hello(state: &State<MyState>) -> &'static str {
//!     // Do things with `state.0`...
//!     "Hello, Postgres!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn rocket(#[cyndra_shared_db::Postgres] pool: PgPool) -> CyndraRocket {
//!     let state = MyState(pool);
//!     let rocket = rocket::build().manage(state).mount("/", routes![hello]);
//!
//!     Ok(rocket.into())
//! }
//! ```
//!
//! For a local run, cyndra will automatically provision a Postgres instance inside a [Docker](https://www.docker.com/) container on your machine and connect it to the `PgPool`.
//!
//! For deploys, cyndra will provision a database for your application and connect it to the `PgPool` on your behalf.
//!
//! To learn more about cyndra managed resources, see our [resource docs](https://docs.cyndra.rs/resources/cyndra-shared-db).
//!
//! ## Configuration
//!
//! The `cargo cyndra` command can be customized by creating a `Cyndra.toml` in the same location as your `Cargo.toml`.
//!
//! ##### Change the name of your service
//!
//! To have your service deployed with a different name, add a `name` entry in the `Cyndra.toml`:
//!
//! ```toml
//! name = "hello-world"
//! ```
//!
//! If the `name` key is not specified, the service's name will be the same as the crate's name.
//!
//! Alternatively, you can override the project name on the command-line, by passing the --name argument to any subcommand like so:
//!
//! ```bash
//! $ cargo cyndra deploy --name=$PROJECT_NAME
//! ```
//!
//! ##### Using Podman instead of Docker
//! If you are using [Podman](https://podman.io/) instead of Docker, then `cargo cyndra run` will give
//! `got unexpected error while inspecting docker container: error trying to connect: No such file or directory` error.
//!
//! To fix this error you will need to expose a rootless socket for Podman first. This can be done using:
//!
//! ```bash
//! podman system service --time=0 unix:///tmp/podman.sock
//! ```
//!
//! Now set the `DOCKER_HOST` environment variable to point to this socket using:
//!
//! ```bash
//! export DOCKER_HOST=unix:///tmp/podman.sock
//! ```
//!
//! Now all `cargo cyndra run` commands will work against Podman.
//!
//! ## Getting API keys
//!
//! After you've installed the [cargo-cyndra](https://docs.rs/crate/cargo-cyndra/latest) command, run:
//!
//! ```bash
//! $ cargo cyndra login
//! ```
//!
//! this will open a browser window and prompt you to connect using your GitHub account.
//!
//! ## We're in alpha 🤗
//!
//! Thanks for using cyndra! We're very happy to have you with us!
//!
//! During our alpha period, API keys are completely free and you can deploy as many services as you want.
//!
//! Just keep in mind that there may be some kinks that require us to take all deployments down once in a while. In certain circumstances we may also have to delete all the data associated with those deployments.
//!
//! To stay updated with the release status of cyndra, [join our Discord](https://discord.gg/cyndra)!
//!
//! ## Join Discord
//!
//! If you have any questions, [join our Discord server](https://discord.gg/cyndra). There's always someone on there that can help!
//!
//! You can also [open an issue or a discussion on GitHub](https://github.com/cyndra-hq/cyndra).
//!

/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// # Without cyndra managed resources
/// The simplest usage is when your service does not require any cyndra managed resources, so you only need to return a cyndra supported service:
///
/// ```rust,no_run
/// use cyndra_rocket::CyndraRocket;
///
/// #[cyndra_rocket::main]
/// async fn rocket() -> CyndraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket.into())
/// }
/// ```
///
/// ## cyndra supported services
/// The following types can be returned from a `#[cyndra_service::main]` function and enjoy first class service support in cyndra.
///
/// | Return type                           | Crate                                                         | Service                                     | Version    | Example                                                                               |
/// | ------------------------------------- |-------------------------------------------------------------- | ------------------------------------------- | ---------- | -----------------------------------------------------------------------------------   |
/// | `CyndraActixWeb`                     |[cyndra-actix-web](https://crates.io/crates/cyndra-actix-web)| [actix-web](https://docs.rs/actix-web/4.3)  | 4.3        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/actix-web/hello-world)      |
/// | `CyndraAxum`                         |[cyndra-axum](https://crates.io/crates/cyndra-axum)          | [axum](https://docs.rs/axum/0.6)            | 0.5        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/axum/hello-world)           |
/// | `CyndraPoem`                         |[cyndra-poem](https://crates.io/crates/cyndra-poem)          | [poem](https://docs.rs/poem/1.3)            | 1.3        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/poem/hello-world)           |
/// | `CyndraPoise`                        |[cyndra-poise](https://crates.io/crates/cyndra-poise)        | [poise](https://docs.rs/poise/0.5)          | 0.5        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/poise/hello-world)          |
/// | `CyndraRocket`                       |[cyndra-rocket](https://crates.io/crates/cyndra-rocket)      | [rocket](https://docs.rs/rocket/0.5.0-rc.2) | 0.5.0-rc.2 | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/rocket/hello-world)         |
/// | `CyndraSalvo`                        |[cyndra-salvo](https://crates.io/crates/cyndra-salvo)        | [salvo](https://docs.rs/salvo/0.37)         | 0.37       | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/salvo/hello-world)          |
/// | `CyndraSerenity`                     |[cyndra-serenity](https://crates.io/crates/cyndra-serenity   | [serenity](https://docs.rs/serenity/0.11)   | 0.11       | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/serenity/hello-world)       |
/// | `CyndraThruster`                     |[cyndra-thruster](https://crates.io/crates/cyndra-thruster)  | [thruster](https://docs.rs/thruster/1.3)    | 1.3        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/thruster/hello-world)       |
/// | `CyndraTower`                        |[cyndra-tower](https://crates.io/crates/cyndra-tower)        | [tower](https://docs.rs/tower/0.4)          | 0.4        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/tower/hello-world)          |
/// | `CyndraTide`                         |[cyndra-tide](https://crates.io/crates/cyndra-tide)          | [tide](https://docs.rs/tide/0.16)           | 0.16       | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/tide/hello-world)           |
///
/// # Getting cyndra managed resources
/// Cyndra is able to manage resource dependencies for you. These resources are passed in as inputs to your `#[cyndra_runtime::main]` function and are configured using attributes:
/// ```rust,no_run
/// use sqlx::PgPool;
/// use cyndra_rocket::CyndraRocket;
///
/// struct MyState(PgPool);
///
/// #[cyndra_runtime::main]
/// async fn rocket(#[cyndra_shared_db::Postgres] pool: PgPool) -> CyndraRocket {
///     let state = MyState(pool);
///     let rocket = rocket::build().manage(state);
///
///     Ok(rocket.into())
/// }
/// ```
///
/// More [cyndra managed resources can be found here](https://github.com/cyndra-hq/cyndra/tree/main/resources)
pub use cyndra_codegen::main;

mod alpha;
mod args;
#[cfg(feature = "next")]
mod next;
mod provisioner_factory;
mod resource_tracker;

pub use alpha::{start, Alpha};
#[cfg(feature = "next")]
pub use next::{AxumWasm, NextArgs};
pub use provisioner_factory::ProvisionerFactory;
pub use resource_tracker::{get_resource, ResourceTracker};
pub use cyndra_common::storage_manager::StorageManager;
pub use cyndra_service::{CustomError, Error, Factory, ResourceBuilder, Service};

pub use async_trait::async_trait;

// Dependencies required by the codegen
pub use anyhow::Context;
pub use strfmt::strfmt;
pub use tracing;

// Print the version of the runtime.
pub fn print_version() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("{name} {version}");
}

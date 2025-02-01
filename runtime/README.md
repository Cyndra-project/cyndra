# Cyndra - Deploy Rust apps with a single Cargo subcommand

<div style="display: flex; margin-top: 30px; margin-bottom: 30px;">
<img src="https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/logo-rectangle-transparent.png" width="400px" style="margin-left: auto; margin-right: auto;"/>
</div>

[Cyndra](https://www.cyndra.rs/) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

📖 Check out our documentation to get started quickly: [docs.cyndra.rs](https://docs.cyndra.rs)

🙋‍♂️ If you have any questions, join our [Discord](https://discord.gg/cyndra) server.

## Usage

Start by installing the [`cargo cyndra`](https://docs.rs/crate/cargo-cyndra/latest) subcommand by running the following in a terminal:

```bash
cargo install cargo-cyndra
```

Now that Cyndra is installed, you can initialize a project with Axum boilerplate:

```bash
cargo cyndra init --template axum my-axum-app
```

By looking at the `Cargo.toml` file of the generated `my-axum-app` project you will see it has been made to
be a binary crate with a few dependencies including `cyndra-runtime` and `cyndra-axum`.

```toml
axum = "0.7.3"
cyndra-axum = "0.44.0"
cyndra-runtime = "0.44.0"
tokio = "1.28.2"
```

A boilerplate code for your axum project can also be found in `src/main.rs`:

```rust,no_run
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

Check out [our docs](https://docs.cyndra.rs/introduction/welcome) to see all the frameworks we support, or
our [examples](https://github.com/cyndra-hq/cyndra-examples) if you prefer that format.

## Running locally

To test your app locally before deploying, use:

```bash
cargo cyndra run
```

You should see your app build and start on the default port 8000. You can test this using;

```bash
curl http://localhost:8000/
# Hello, world!
```

## Deploying

Before you can deploy, you have to create a project. This will start a deployer container for your
project under the hood, ensuring isolation from other users' projects. PS. you don't have to do this
now if you did in in the `cargo cyndra init` flow.

```bash
cargo cyndra project start
```

Then, deploy the service with:

```bash
cargo cyndra deploy
```

Your service will immediately be available at `https://{project_name}.cyndraapp.rs/`. For example:

```bash
curl https://my-axum-app.cyndraapp.rs/
# Hello, world!
```

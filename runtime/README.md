# Cyndra - Deploy Rust apps with a single command

<div style="display: flex; margin-top: 30px; margin-bottom: 30px;">
<img src="https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/logo-rectangle-transparent.png" width="400px" style="margin-left: auto; margin-right: auto;"/>
</div>

[Cyndra](https://www.cyndra.dev/) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

📖 Check out our documentation to get started quickly: [docs.cyndra.dev](https://docs.cyndra.dev)

🙋‍♂️ If you have any questions, join our [Discord](https://discord.gg/cyndra) server.

## Usage

Start by installing the [Cyndra CLI](https://crates.io/crates/cargo-cyndra) by running the following in a terminal ([more installation options](https://docs.cyndra.dev/getting-started/installation)):

```bash
# Linux / macOS
curl -sSfL https://www.cyndra.dev/install | bash

# Windows (Powershell)
iwr https://www.cyndra.dev/install-win | iex
```

Now that Cyndra is installed, you can initialize a project with Axum boilerplate:

```bash
cyndra init --template axum my-axum-app
```

By looking at the `Cargo.toml` file of the generated `my-axum-app` project you will see it has been made to
be a binary crate with a few dependencies including `cyndra-runtime` and `cyndra-axum`.

```toml
axum = "0.8.1"
cyndra-axum = "0.55.0"
cyndra-runtime = "0.55.0"
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

Check out [our docs](https://docs.cyndra.dev) to see all the frameworks we support, or
our [examples](https://github.com/cyndra-hq/cyndra-examples) if you prefer that format.

## Running locally

To test your app locally before deploying, use:

```bash
cyndra run
```

You should see your app build and start on the default port 8000. You can test this using;

```bash
curl http://localhost:8000/
# Hello, world!
```

## Deploying

Deploy the service with:

```bash
cyndra deploy
```

Your service will then be made available under a subdomain of `*.cyndra.app`. For example:

```bash
curl https://my-axum-app-0000.cyndra.app/
# Hello, world!
```

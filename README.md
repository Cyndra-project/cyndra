<!-- markdownlint-disable -->
<p align="center">
<img width="700" src="https://github.com/user-attachments/assets/1cc0f346-abd8-4753-ac5f-ad918bc87c46"/>
</p>
<br>

<h1 align="center">Swiftly Create & Launch Rust Applications</h1>
<div align="center">
Provision resources and deploy your projects with minimal code.
</div>

<h3 align="center">Effortless. Streamlined. Delightful.</h3>

<div align="center"><img src="https://github.com/user-attachments/assets/d85de8aa-2902-447b-9fdd-6ab232342c32" width="100%" ></div>
<br>

*<div align="center">🌟 If you like Cyndra, please give this repository a star to share it with others.</div>*
<br>

## Key Features

- **Single-Line Resource Setup:** Add a database or other resources with just one line in your main file—no need for configuration or YAML files.
- **Fast Development Cycle:** Go from project setup to deployment in just 2 minutes. Provision resources in seconds and push to production effortlessly.
- **Top Rust Frameworks Supported:** Full compatibility with [Axum](https://docs.cyndra.dev/examples/axum), [Actix Web](https://docs.cyndra.dev/examples/actix), [Rocket](https://docs.cyndra.dev/examples/rocket), and [more](https://docs.cyndra.dev/examples/other).
- **Secure by Default:** Focus on coding while we handle security and permissions for you.
<br>
<br>

## Getting Started

For Linux or macOS, run this installation script to automatically set up the appropriate version for your system:

```sh
curl -sSfL https://www.cyndra.dev/install | bash
```

For Windows, use the following script to achieve the same:

```powershell
iwr "https://www.cyndra.dev/install-win" | iex
```

Once installed, log in with:

```sh
cyndra login
```

To start a new project, run:

```bash
cyndra init --template axum hello-world
```

To deploy it, navigate to the project folder and execute:

```bash
cd hello-world
cyndra deploy
```

That’s all it takes!

```text
Service Name:  hello-world
Deployment ID: 3d08ac34-ad63-41c1-836b-99afdc90af9f
Status:        active
Last Updated:  2022-04-01T08:32:34Z
URI:           https://hello-world.cyndraapp.rs
```

You can build upon the `hello-world` template or explore our [example projects](https://github.com/cyndra-project/cyndra-examples).

For comprehensive guides, check out [our documentation](https://docs.cyndra.dev).
<br>
<br>

## Example Overview

Here’s a simple "Hello World" app using Axum:

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

To make it deployable with a single command, modify it like this:

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

Now, running `cyndra deploy` will make your app live. To add a shared Postgres database, update the code as follows:

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

Run `cyndra deploy` again, and your project will be live with a fully configured database.
<br>
<br>

## Repositories

| Name | Description |
|-|-|
| [cyndra](https://github.com/cyndra-project/cyndra) 🚀 (This repo) | Contains all library crates and the Cyndra CLI tool. |
| [cyndra-examples](https://github.com/cyndra-project/cyndra-examples) 👨‍🏫 | Official example projects deployable on Cyndra. |
| [cyndra-docs](https://docs.cyndra.dev/) 📃 | Official documentation at [docs.cyndra.dev](https://docs.cyndra.dev/). |
<br>

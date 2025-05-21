<!-- markdownlint-disable -->
<p align="center">
<img width="700" src="https://github.com/user-attachments/assets/1cc0f346-abd8-4753-ac5f-ad918bc87c46"/>
</p>
<br>

<h1 align="center">Fastest Way to Build & Ship Rust Apps</h1>
<div align="center">
Get resources and deploy your apps with a few lines of code.
</div>

<h3 align="center">Simple. &nbsp; Easy. &nbsp; Joyful.</h3>

<div align="center"><img src="https://i.imgur.com/1qdWipP.gif" width="100%" ></div>
<br>


*<div align="center">⭐ If you find Cyndra interesting, consider starring this repo to help spread the word.</div>*
<br>

## Features

- **One-line Resource Provisioning:** Get a database or other resource by adding a single line of code to your main file. No config or yaml files required.
- **Rapid Development:** It takes 2 minutes from project initialization to a deployed project. It takes seconds to provision a resource, and get it deployed to production.
- **First-class support for popular Rust frameworks:** [Axum](https://docs.cyndra.dev/examples/axum), [Actix Web](https://docs.cyndra.dev/examples/actix), [Rocket](https://docs.cyndra.dev/examples/rocket), and [more](https://docs.cyndra.dev/examples/other)
- **Security:** Let us worry about the security & permissions while you focus on writing good code.
<br>
<br>

## Quick Start

On Linux and macOS, you can use this install script, which will automatically install the correct target for your OS and distro:

```sh
curl -sSfL https://www.cyndra.dev/install | bash
```

On Windows, you can use this install script to do the same:

```powershell
iwr "https://www.cyndra.dev/install-win" | iex
```

After installing, log in with:

```sh
cyndra login
```

To initialize your project, simply write:

```bash
cyndra init --template axum hello-world
```

And to deploy it, write:

```bash
cd hello-world
cyndra deploy
```

And... that's it!

```text
Service Name:  hello-world
Deployment ID: 3d08ac34-ad63-41c1-836b-99afdc90af9f
Status:        running
Last Updated:  2022-04-01T08:32:34Z
URI:           https://hello-world.cyndraapp.rs
```

Feel free to build on top of the generated `hello-world` boilerplate or take a stab at one of our [examples](https://github.com/cyndra-project/cyndra-examples).

For the full documentation, visit [our docs](https://docs.cyndra.dev).
<br>
<br>

## Quick Look

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

Now, with just `cyndra deploy`, you can see your application live. But let's enhance it further by adding a shared Postgres database:

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

Now, if we run `cyndra deploy`, we'll have an up and running project with a database inside & ready to use.
<br>
<br>

## Repositories

| Name | Description |
|-|-|
| [cyndra](https://github.com/cyndra-project/cyndra) 🚀 (This repo) | All library crates and the Cyndra CLI. |
| [cyndra-examples](https://github.com/cyndra-project/cyndra-examples) 👨‍🏫 | Officially maintained examples of projects that can be deployed on Cyndra. |
| [cyndra-docs](https://docs.cyndra.dev/) 📃 | Official documentation [docs.cyndra.dev](https://docs.cyndra.dev/). |
<br>


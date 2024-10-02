<div align="center">

# cargo-cyndra

<p align=center>
  <a href="https://github.com/cyndra-hq/cyndra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://github.com/cyndra-hq/cyndra/actions">
    <img alt="build status" src="https://img.shields.io/github/workflow/status/cyndra-hq/cyndra/cargo-test"/>
  </a>
  <a href="https://discord.gg/H33rRDTm3p">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
</p>

`cargo-cyndra` is your commandline tool for deploying web apps on [cyndra](https://www.cyndra.rs/), the stateful serverless web platform for Rust.

**README Sections:** [Installation](#installation) — [Subcommands](#subcommands) — [Development](#development)

</div>

---

`cargo-cyndra` brings [cyndra](https://www.cyndra.rs/), the open source serverless platform for Rust web applications, into your terminal. With a dedicated focus on productivity, reliability, and performance, `cargo-cyndra` makes deploying your code to the cloud as easy as deriving a trait.

---

<a id="installation">
<h1>Installation</h1>
</a>

`cargo-cyndra` is available for macOS, Linux, and Windows. To install the commandline tool, run:

```sh
$ cargo install cargo-cyndra
```

---

<a id="subcommands">
<h1>Subcommands</h1>
</a>

`cargo-cyndra`'s subcommands help you build and deploy web apps from start to finish.

Run `cargo-cyndra --help` to see the basic usage:

```
USAGE:
    cargo-cyndra [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --api-url <API_URL>                        run this command against the api at the supplied url (allows targeting a custom deployed instance for this command only) [env: cyndra_API=]
    -h, --help                                     Print help information
        --name <NAME>                              Specify the name of the project (overrides crate name)
    -V, --version                                  Print version information
        --working-directory <WORKING_DIRECTORY>    Specify the working directory [default: .]

SUBCOMMANDS:
    auth          create user credentials for the cyndra platform
    delete        delete this cyndra service
    deploy        deploy a cyndra service
    deployment    manage deployments of a cyndra service
    help          Print this message or the help of the given subcommand(s)
    init          create a new cyndra service
    login         login to the cyndra platform
    logs          view the logs of a deployment in this cyndra service
    run           run a cyndra service locally
    status        view the status of a cyndra service
```

### Subcommand: `init`

To initialize a cyndra project with boilerplates, run `cargo cyndra init [OPTIONS] [PATH]`. 

Currently, `cargo cyndra init` supports the following frameworks:

- `--axum`: for [axum](https://github.com/tokio-rs/axum) framework
- `--actix-web`: for [actix web](https://actix.rs/) framework
- `--poem`: for [poem](https://github.com/poem-web/poem) framework
- `--rocket`: for [rocket](https://rocket.rs/) framework
- `--salvo`: for [salvo](https://salvo.rs/) framework
- `--serenity`: for [serenity](https://serenity.rs/) discord bot framework
- `--thruster`: for [thruster](https://github.com/thruster-rs/Thruster) framework
- `--tide`: for [tide](https://github.com/http-rs/tide) framework
- `--tower`: for [tower](https://github.com/tower-rs/tower) library
- `--warp`: for [warp](https://github.com/seanmonstar/warp) framework

For example, running the following command will initialize a project for [rocket](https://rocket.rs/):

```sh
$ cargo cyndra init --rocket my-rocket-app
```

This should generate the following dependency in `Cargo.toml`:
```toml
cyndra-service = { version = "0.9.0", features = ["web-rocket"] }
```

The following boilerplate code should be generated into `src/lib.rs`:

```rust
#[macro_use]
extern crate rocket;

use cyndra_service::CyndraRocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[cyndra_service::main]
async fn init() -> CyndraRocket {
    let rocket = rocket::build().mount("/", routes![hello]);

    Ok(rocket)
}
```

### Subcommand: `run`

To run the cyndra project locally, use the following command:

```sh
# Inside your cyndra project
$ cargo cyndra run
```

This will compile your cyndra project and start it on the default port `8000`. Test it by:

```sh
$ curl http://localhost:8000/hello
Hello, world!
```

### Subcommand: `login`

Use `cargo cyndra login` inside your cyndra project to generate an API key for the cyndra platform:

```sh
# Inside a cyndra project
$ cargo cyndra login
```

This should automatically open a browser window with an auto-generated API key for your project. Simply copy-paste the API key back in your terminal or run the following command to complete login:

```sh
$ cargo cyndra login --api-key your-api-key-from-browser
```

### Subcommand: `deploy`

To deploy your cyndra project to the cloud, run:

```sh
$ cargo cyndra project new
$ cargo cyndra deploy
```

Your service will immediately be available at `{crate_name}.cyndraapp.rs`. For instance:

```sh
$ curl https://my-rocket-app.cyndraapp.rs/hello
Hello, world!
```

### Subcommand: `status`

Check the status of your deployed cyndra project with:

```sh
$ cargo cyndra status
```

### Subcommand: `logs`

Check the logs of your deployed cyndra project with:

```sh
$ cargo cyndra logs
```

### Subcommand: `delete`

Once you are done with a deployment, you can delete it by running:

```sh
$ cargo cyndra delete
```

---

<a id="development">
<h1>Development</h1>
</a>

Thanks for using `cargo-cyndra`! We’re very happy to have you with us!

During our alpha period, API keys are completely free and you can deploy as many services as you want.

Just keep in mind that there may be some kinks that require us to take all deployments down once in a while. In certain circumstances we may also have to delete all the data associated with those deployments.

To contribute to `cargo-cyndra` or stay updated with our development, please [open an issue, discussion or PR on Github](https://github.com/cyndra-hq/cyndra) and [join our Discord](https://discord.gg/H33rRDTm3p)! 🚀

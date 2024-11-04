<!-- markdownlint-disable -->
<div align="center">

# cargo-cyndra

<p align=center>
  <a href="https://docs.rs/cyndra-service">
    <img alt="docs" src="https://img.shields.io/badge/docs-reference-orange">
  </a>
  <a href="https://github.com/cyndra-hq/cyndra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://circleci.com/gh/cyndra-hq/cyndra/">
    <img alt="build status" src="https://circleci.com/gh/cyndra-hq/cyndra.svg?style=shield"/>
  </a>
  <a href="https://discord.gg/cyndra">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
</p>
<!-- markdownlint-restore -->
<!-- markdownlint-disable MD001 -->

`cargo-cyndra` is your commandline tool for deploying web apps on [cyndra](https://www.cyndra.rs/), the stateful serverless web platform for Rust.

**README Sections:** [Installation](#installation) — [Subcommands](#subcommands) — [Development](#development)

</div>

---

`cargo-cyndra` brings [cyndra](https://www.cyndra.rs/), the open source serverless platform for Rust web applications, into your terminal. With a dedicated focus on productivity, reliability, and performance, `cargo-cyndra` makes deploying your code to the cloud as easy as deriving a trait.

---

<!-- markdownlint-disable-next-line -->
<a id="installation"><h1>Installation</h1></a>

`cargo-cyndra` is available for macOS, Linux, and Windows. To install the commandline tool, run:

```bash
cargo install cargo-cyndra
```

### Distro Packages

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/cargo-cyndra.svg)](https://repology.org/project/cargo-cyndra/versions)

</details>

#### Arch Linux

`cargo-cyndra` can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/cargo-cyndra) using [pacman](https://wiki.archlinux.org/title/Pacman):

```sh
pacman -S cargo-cyndra
```

---

<!-- markdownlint-disable-next-line -->
<a id="subcommands"><h1>Subcommands</h1></a>

`cargo-cyndra`'s subcommands help you build and deploy web apps from start to finish.

Run `cargo cyndra help` to see the basic usage:

```text
Usage: cargo-cyndra [OPTIONS] <COMMAND>

Commands:
  init        Create a new cyndra project
  run         Run a cyndra service locally
  deploy      Deploy a cyndra service
  deployment  Manage deployments of a cyndra service
  status      View the status of a cyndra service
  stop        Stop this cyndra service
  logs        View the logs of a deployment in this cyndra service
  project     List or manage projects on cyndra
  resource    Manage resources of a cyndra project
  secrets     Manage secrets for this cyndra service
  clean       Remove cargo build artifacts in the cyndra environment
  login       Login to the cyndra platform
  logout      Log out of the cyndra platform
  generate    Generate shell completions
  feedback    Open an issue on GitHub and provide feedback
  help        Print this message or the help of the given subcommand(s)

Options:
      --working-directory <WORKING_DIRECTORY>  Specify the working directory [default: .]
      --name <NAME>                            Specify the name of the project (overrides crate name)
      --api-url <API_URL>                      Run this command against the API at the supplied URL (allows targeting a custom deployed instance for this command only, mainly
                                               for development) [env: cyndra_API=]
  -h, --help                                   Print help
  -V, --version                                Print version
```

### Subcommand: `init`

To initialize a cyndra project with boilerplates, run `cargo cyndra init [OPTIONS] [PATH]`.

Currently, `cargo cyndra init` supports the following frameworks:

- `--template actix-web`: for [actix web](https://actix.rs/) framework
- `--template axum`: for [axum](https://github.com/tokio-rs/axum) framework
- `--template poem`: for [poem](https://github.com/poem-web/poem) framework
- `--template poise`: for [poise](https://github.com/serenity-rs/poise) discord bot framework
- `--template rocket`: for [rocket](https://rocket.rs/) framework
- `--template salvo`: for [salvo](https://salvo.rs/) framework
- `--template serenity`: for [serenity](https://github.com/serenity-rs/serenity) discord bot framework
- `--template thruster`: for [thruster](https://github.com/thruster-rs/Thruster) framework
- `--template tide`: for [tide](https://github.com/http-rs/tide) framework
- `--template tower`: for [tower](https://github.com/tower-rs/tower) library
- `--template warp`: for [warp](https://github.com/seanmonstar/warp) framework

For example, running the following command will initialize a project for [rocket](https://rocket.rs/):

```sh
cargo cyndra init --template rocket my-rocket-app
```

This should generate the following dependency in `Cargo.toml`:

```toml
rocket = "0.5.0-rc.2"
cyndra-rocket = { version = "0.17.0" }
cyndra-runtime = { version = "0.17.0" }
tokio = { version = "1.26.0" }
```

The following boilerplate code should be generated into `src/lib.rs`:

```rust
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn rocket() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
```

### Subcommand: `run`

To run the cyndra project locally, use the following command:

```sh
# Inside your cyndra project
cargo cyndra run
```

This will compile your cyndra project and start it on the default port `8000`. Test it by:

```sh
$ curl http://localhost:8000
Hello, world!
```

### Subcommand: `login`

Use `cargo cyndra login` inside your cyndra project to generate an API key for the cyndra platform:

```sh
# Inside a cyndra project
cargo cyndra login
```

This should automatically open a browser window with an auto-generated API key for your project. Simply copy-paste the API key back in your terminal or run the following command to complete login:

```sh
cargo cyndra login --api-key <your-api-key-from-browser>
```

### Subcommand: `deploy`

To deploy your cyndra project to the cloud, run:

```sh
cargo cyndra project start
cargo cyndra deploy
```

Your service will immediately be available at `{crate_name}.cyndraapp.rs`. For instance:

```sh
$ curl https://my-rocket-app.cyndraapp.rs
Hello, world!
```

### Subcommand: `status`

Check the status of your deployed cyndra project with:

```sh
cargo cyndra status
```

### Subcommand: `logs`

Check the logs of your deployed cyndra project with:

```sh
cargo cyndra logs
```

### Subcommand: `stop`

Once you are done with a deployment, you can stop it by running:

```sh
cargo cyndra stop
```

---

<!-- markdownlint-disable-next-line -->
<a id="development"><h1>Development</h1></a>

Thanks for using `cargo-cyndra`! We’re very happy to have you with us!

During our alpha period, API keys are completely free and you can deploy as many services as you want.

Just keep in mind that there may be some kinks that require us to take all deployments down once in a while. In certain circumstances we may also have to delete all the data associated with those deployments.

To contribute to `cargo-cyndra` or stay updated with our development, please [open an issue, discussion or PR on Github](https://github.com/cyndra-hq/cyndra) and [join our Discord](https://discord.gg/cyndra)! 🚀

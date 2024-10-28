use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use cargo_edit::{find, get_latest_dependency, registry_url};
use indoc::indoc;
use cyndra_common::project::ProjectName;
use toml_edit::{value, Array, Document, Table};
use url::Url;

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Template {
    ActixWeb,
    Axum,
    Poise,
    Poem,
    Rocket,
    Salvo,
    Serenity,
    Tide,
    Thruster,
    Tower,
    Warp,
    None,
}

impl Template {
    /// Returns a framework-specific struct that implements the trait `CyndraInit`
    /// for writing framework-specific dependencies to `Cargo.toml` and generating
    /// boilerplate code in `src/main.rs`.
    pub fn init_config(&self) -> Box<dyn CyndraInit> {
        use Template::*;
        match self {
            ActixWeb => Box::new(CyndraInitActixWeb),
            Axum => Box::new(CyndraInitAxum),
            Rocket => Box::new(CyndraInitRocket),
            Tide => Box::new(CyndraInitTide),
            Tower => Box::new(CyndraInitTower),
            Poem => Box::new(CyndraInitPoem),
            Salvo => Box::new(CyndraInitSalvo),
            Serenity => Box::new(CyndraInitSerenity),
            Poise => Box::new(CyndraInitPoise),
            Warp => Box::new(CyndraInitWarp),
            Thruster => Box::new(CyndraInitThruster),
            None => Box::new(CyndraInitNoOp),
        }
    }
}

pub trait CyndraInit {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    );
    fn get_boilerplate_code_for_framework(&self) -> &'static str;
}

pub struct CyndraInitActixWeb;

impl CyndraInit for CyndraInitActixWeb {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "actix-web",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-actix-web",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use actix_web::{get, web::ServiceConfig};
        use cyndra_actix_web::CyndraActixWeb;

        #[get("/")]
        async fn hello_world() -> &'static str {
            "Hello World!"
        }

        #[cyndra_runtime::main]
        async fn actix_web(
        ) -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
            let config = move |cfg: &mut ServiceConfig| {
                cfg.service(hello_world);
            };

            Ok(config.into())
        }"#}
    }
}

pub struct CyndraInitAxum;

impl CyndraInit for CyndraInitAxum {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "axum",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-axum",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use axum::{routing::get, Router};

        async fn hello_world() -> &'static str {
            "Hello, world!"
        }

        #[cyndra_runtime::main]
        async fn axum() -> cyndra_axum::CyndraAxum {
            let router = Router::new().route("/", get(hello_world));

            Ok(router.into())
        }"#}
    }
}

pub struct CyndraInitRocket;

impl CyndraInit for CyndraInitRocket {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "rocket",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-rocket",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
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
        }"#}
    }
}

pub struct CyndraInitTide;

impl CyndraInit for CyndraInitTide {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "cyndra-tide",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tide",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        #[cyndra_runtime::main]
        async fn tide() -> cyndra_tide::CyndraTide<()> {
            let mut app = tide::new();
            app.with(tide::log::LogMiddleware::new());

            app.at("/").get(|_| async { Ok("Hello, world!") });

            Ok(app.into())
        }"#}
    }
}

pub struct CyndraInitPoem;

impl CyndraInit for CyndraInitPoem {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "poem",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-poem",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use poem::{get, handler, Route};
        use cyndra_poem::CyndraPoem;

        #[handler]
        fn hello_world() -> &'static str {
            "Hello, world!"
        }

        #[cyndra_runtime::main]
        async fn poem() -> CyndraPoem<impl poem::Endpoint> {
            let app = Route::new().at("/", get(hello_world));

            Ok(app.into())
        }"#}
    }
}

pub struct CyndraInitSalvo;

impl CyndraInit for CyndraInitSalvo {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "salvo",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-salvo",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use salvo::prelude::*;

        #[handler]
        async fn hello_world(res: &mut Response) {
            res.render(Text::Plain("Hello, world!"));
        }

        #[cyndra_runtime::main]
        async fn salvo() -> cyndra_salvo::CyndraSalvo {
            let router = Router::with_path("hello").get(hello_world);

            Ok(router.into())
        }"#}
    }
}

pub struct CyndraInitSerenity;

impl CyndraInit for CyndraInitSerenity {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "anyhow",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_version(
            "serenity",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        dependencies["serenity"]["default-features"] = value(false);

        set_inline_table_dependency_features(
            "serenity",
            dependencies,
            vec![
                "client".into(),
                "gateway".into(),
                "rustls_backend".into(),
                "model".into(),
            ],
        );

        set_key_value_dependency_version(
            "cyndra-secrets",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-serenity",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tracing",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use anyhow::anyhow;
        use serenity::async_trait;
        use serenity::model::channel::Message;
        use serenity::model::gateway::Ready;
        use serenity::prelude::*;
        use cyndra_secrets::SecretStore;
        use tracing::{error, info};

        struct Bot;

        #[async_trait]
        impl EventHandler for Bot {
            async fn message(&self, ctx: Context, msg: Message) {
                if msg.content == "!hello" {
                    if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                        error!("Error sending message: {:?}", e);
                    }
                }
            }

            async fn ready(&self, _: Context, ready: Ready) {
                info!("{} is connected!", ready.user.name);
            }
        }

        #[cyndra_runtime::main]
        async fn serenity(
            #[cyndra_secrets::Secrets] secret_store: SecretStore,
        ) -> cyndra_serenity::CyndraSerenity {
            // Get the discord token set in `Secrets.toml`
            let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
                token
            } else {
                return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
            };

            // Set gateway intents, which decides what events the bot will be notified about
            let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

            let client = Client::builder(&token, intents)
                .event_handler(Bot)
                .await
                .expect("Err creating client");

            Ok(client.into())
        }"#}
    }
}

pub struct CyndraInitPoise;

impl CyndraInit for CyndraInitPoise {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "anyhow",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "poise",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-poise",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "cyndra-secrets",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tracing",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use anyhow::Context as _;
        use poise::serenity_prelude as serenity;
        use cyndra_secrets::SecretStore;
        use cyndra_poise::CyndraPoise;

        struct Data {} // User data, which is stored and accessible in all command invocations
        type Error = Box<dyn std::error::Error + Send + Sync>;
        type Context<'a> = poise::Context<'a, Data, Error>;

        /// Responds with "world!"
        #[poise::command(slash_command)]
        async fn hello(ctx: Context<'_>) -> Result<(), Error> {
            ctx.say("world!").await?;
            Ok(())
        }

        #[cyndra_runtime::main]
        async fn poise(#[cyndra_secrets::Secrets] secret_store: SecretStore) -> CyndraPoise<Data, Error> {
            // Get the discord token set in `Secrets.toml`
            let discord_token = secret_store
                .get("DISCORD_TOKEN")
                .context("'DISCORD_TOKEN' was not found")?;

            let framework = poise::Framework::builder()
                .options(poise::FrameworkOptions {
                    commands: vec![hello()],
                    ..Default::default()
                })
                .token(discord_token)
                .intents(serenity::GatewayIntents::non_privileged())
                .setup(|ctx, _ready, framework| {
                    Box::pin(async move {
                        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                        Ok(Data {})
                    })
                })
                .build()
                .await
                .map_err(cyndra_runtime::CustomError::new)?;

            Ok(framework.into())
        }"#}
    }
}

pub struct CyndraInitTower;

impl CyndraInit for CyndraInitTower {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_inline_table_dependency_version(
            "hyper",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_features("hyper", dependencies, vec!["full".to_string()]);

        set_key_value_dependency_version(
            "cyndra-tower",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_version(
            "tower",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_features("tower", dependencies, vec!["full".to_string()]);
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use std::convert::Infallible;
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};

        #[derive(Clone)]
        struct HelloWorld;

        impl tower::Service<hyper::Request<hyper::Body>> for HelloWorld {
            type Response = hyper::Response<hyper::Body>;
            type Error = Infallible;
            type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, _req: hyper::Request<hyper::Body>) -> Self::Future {
                let body = hyper::Body::from("Hello, world!");
                let resp = hyper::Response::builder()
                    .status(200)
                    .body(body)
                    .expect("Unable to create the `hyper::Response` object");

                let fut = async { Ok(resp) };

                Box::pin(fut)
            }
        }

        #[cyndra_runtime::main]
        async fn tower() -> cyndra_tower::CyndraTower<HelloWorld> {
            let service = HelloWorld;

            Ok(service.into())
        }"#}
    }
}

pub struct CyndraInitWarp;

impl CyndraInit for CyndraInitWarp {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "cyndra-warp",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_key_value_dependency_version(
            "warp",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use warp::Filter;
        use warp::Reply;
        
        #[cyndra_runtime::main]
        async fn warp() -> cyndra_warp::CyndraWarp<(impl Reply,)> {
            let route = warp::any().map(|| "Hello, World!");
            Ok(route.boxed().into())
        }"#}
    }
}

pub struct CyndraInitThruster;

impl CyndraInit for CyndraInitThruster {
    fn set_cargo_dependencies(
        &self,
        dependencies: &mut Table,
        manifest_path: &Path,
        url: &Url,
        get_dependency_version_fn: GetDependencyVersionFn,
    ) {
        set_key_value_dependency_version(
            "cyndra-thruster",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_version(
            "thruster",
            dependencies,
            manifest_path,
            url,
            false,
            get_dependency_version_fn,
        );

        set_inline_table_dependency_features(
            "thruster",
            dependencies,
            vec!["hyper_server".to_string()],
        );

        set_key_value_dependency_version(
            "tokio",
            dependencies,
            manifest_path,
            url,
            true,
            get_dependency_version_fn,
        );
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        indoc! {r#"
        use thruster::{
            context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest},
            m, middleware_fn, App, HyperServer, MiddlewareNext, MiddlewareResult, ThrusterServer,
        };
        
        #[middleware_fn]
        async fn hello(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
            context.body("Hello, World!");
            Ok(context)
        }
        
        #[cyndra_runtime::main]
        async fn thruster() -> cyndra_thruster::CyndraThruster<HyperServer<Ctx, ()>> {
            let server = HyperServer::new(
                App::<HyperRequest, Ctx, ()>::create(generate_context, ()).get("/", m![hello]),
            );
            
            Ok(server.into())
        }"#}
    }
}

pub struct CyndraInitNoOp;
impl CyndraInit for CyndraInitNoOp {
    fn set_cargo_dependencies(
        &self,
        _dependencies: &mut Table,
        _manifest_path: &Path,
        _url: &Url,
        _get_dependency_version_fn: GetDependencyVersionFn,
    ) {
    }

    fn get_boilerplate_code_for_framework(&self) -> &'static str {
        ""
    }
}

pub fn cargo_init(path: PathBuf, name: ProjectName) -> Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("init")
        .arg("--bin")
        .arg("--name")
        .arg(name.as_str())
        .arg(path.as_os_str());
    println!(r#"    Creating project "{name}" in {path:?}"#);
    let output = cmd.output().expect("Failed to initialize with cargo init.");
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        bail!("cargo init failed:\n{}", stderr)
    }
    print!("{}", stderr);

    Ok(())
}

/// Performs cyndra init on the existing files generated by `cargo init [path]`.
pub fn cargo_cyndra_init(path: PathBuf, framework: Template) -> Result<()> {
    println!(r#"     Setting up "{framework}" template"#);
    let cargo_toml_path = path.join("Cargo.toml");
    let mut cargo_doc = read_to_string(cargo_toml_path.clone())
        .unwrap()
        .parse::<Document>()
        .unwrap();

    // Add publish: false to avoid accidental `cargo publish`
    cargo_doc["package"]["publish"] = value(false);

    // Get `[dependencies]` table
    let dependencies = cargo_doc["dependencies"]
        .as_table_mut()
        .expect("manifest to have a dependencies table");

    let manifest_path = find(Some(path.as_path())).unwrap();
    let url = registry_url(manifest_path.as_path(), None).expect("Could not find registry URL");

    let init_config = framework.init_config();

    set_key_value_dependency_version(
        "cyndra-runtime",
        dependencies,
        &manifest_path,
        &url,
        true, // TODO: disallow pre-release when releasing 0.12?
        get_latest_dependency_version,
    );

    // Set framework-specific dependencies to the `dependencies` table
    init_config.set_cargo_dependencies(
        dependencies,
        &manifest_path,
        &url,
        get_latest_dependency_version,
    );

    // Truncate Cargo.toml and write the updated `Document` to it
    let mut cargo_toml = File::create(cargo_toml_path)?;

    cargo_toml.write_all(cargo_doc.to_string().as_bytes())?;

    // Write boilerplate to `src/main.rs` file
    let main_path = path.join("src").join("main.rs");
    let boilerplate = init_config.get_boilerplate_code_for_framework();
    if !boilerplate.is_empty() {
        write_main_file(boilerplate, &main_path)?;
    }

    Ok(())
}

/// Sets dependency version for a key-value pair:
/// `crate_name = "version"`
fn set_key_value_dependency_version(
    crate_name: &str,
    dependencies: &mut Table,
    manifest_path: &Path,
    url: &Url,
    flag_allow_prerelease: bool,
    get_dependency_version_fn: GetDependencyVersionFn,
) {
    let dependency_version =
        get_dependency_version_fn(crate_name, flag_allow_prerelease, manifest_path, url);
    dependencies[crate_name] = value(dependency_version);
}

/// Sets dependency version for an inline table:
/// `crate_name = { version = "version" }`
fn set_inline_table_dependency_version(
    crate_name: &str,
    dependencies: &mut Table,
    manifest_path: &Path,
    url: &Url,
    flag_allow_prerelease: bool,
    get_dependency_version_fn: GetDependencyVersionFn,
) {
    let dependency_version =
        get_dependency_version_fn(crate_name, flag_allow_prerelease, manifest_path, url);
    dependencies[crate_name]["version"] = value(dependency_version);
}

/// Sets dependency features for an inline table:
/// `crate_name = { features = ["some-feature"] }`
fn set_inline_table_dependency_features(
    crate_name: &str,
    dependencies: &mut Table,
    features: Vec<String>,
) {
    let features = Array::from_iter(features);
    dependencies[crate_name]["features"] = value(features);
}

/// Abstract type for `get_latest_dependency_version` function.
type GetDependencyVersionFn = fn(&str, bool, &Path, &Url) -> String;

/// Gets the latest version for a dependency of `crate_name`.
/// This is a wrapper function for `cargo_edit::get_latest_dependency` function.
fn get_latest_dependency_version(
    crate_name: &str,
    flag_allow_prerelease: bool,
    manifest_path: &Path,
    url: &Url,
) -> String {
    let latest_version =
        get_latest_dependency(crate_name, flag_allow_prerelease, manifest_path, Some(url))
            .unwrap_or_else(|_| panic!("Could not query the latest version of {crate_name}"));
    let latest_version = latest_version
        .version()
        .expect("No latest cyndra-service version available");

    latest_version.to_string()
}

/// Writes `boilerplate` code to the specified `main.rs` file path.
pub fn write_main_file(boilerplate: &'static str, main_path: &Path) -> Result<()> {
    let mut main_file = File::create(main_path)?;
    main_file.write_all(boilerplate.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod cyndra_init_tests {
    use super::*;

    fn cargo_toml_factory() -> Document {
        indoc! {r#"
            [dependencies]
        "#}
        .parse::<Document>()
        .unwrap()
    }

    fn mock_get_latest_dependency_version(
        _crate_name: &str,
        _flag_allow_prerelease: bool,
        _manifest_path: &Path,
        _url: &Url,
    ) -> String {
        "1.0".to_string()
    }

    #[test]
    fn test_set_inline_table_dependency_features() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();

        set_inline_table_dependency_features(
            "cyndra-service",
            dependencies,
            vec!["builder".to_string()],
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-service = { features = ["builder"] }
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_inline_table_dependency_version() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_inline_table_dependency_version(
            "cyndra-service",
            dependencies,
            &manifest_path,
            &url,
            false,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-service = { version = "1.0" }
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_key_value_dependency_version() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-service",
            dependencies,
            &manifest_path,
            &url,
            false,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-service = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }
    #[test]
    fn test_set_cargo_dependencies_actix_web() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitActixWeb.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            actix-web = "1.0"
            cyndra-actix-web = "1.0"
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_axum() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitAxum.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            axum = "1.0"
            cyndra-axum = "1.0"
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_rocket() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitRocket.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            rocket = "1.0"
            cyndra-rocket = "1.0"
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_tide() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitTide.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            cyndra-tide = "1.0"
            tokio = "1.0"
            tide = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_tower() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitTower.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            hyper = { version = "1.0", features = ["full"] }
            cyndra-tower = "1.0"
            tokio = "1.0"
            tower = { version = "1.0", features = ["full"] }
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_poem() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitPoem.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            poem = "1.0"
            cyndra-poem = "1.0"
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_salvo() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitSalvo.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            salvo = "1.0"
            cyndra-salvo = "1.0"
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_serenity() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitSerenity.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            anyhow = "1.0"
            serenity = { version = "1.0", default-features = false, features = ["client", "gateway", "rustls_backend", "model"] }
            cyndra-secrets = "1.0"
            cyndra-serenity = "1.0"
            tokio = "1.0"
            tracing = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_poise() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitPoise.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            anyhow = "1.0"
            poise = "1.0"
            cyndra-poise = "1.0"
            cyndra-secrets = "1.0"
            tokio = "1.0"
            tracing = "1.0"
		"#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_warp() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitWarp.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            cyndra-warp = "1.0"
            tokio = "1.0"
            warp = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    #[test]
    fn test_set_cargo_dependencies_thruster() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://cyndra.rs").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitThruster.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            mock_get_latest_dependency_version,
        );

        let expected = indoc! {r#"
            [dependencies]
            cyndra-runtime = "1.0"
            cyndra-thruster = "1.0"
            thruster = { version = "1.0", features = ["hyper_server"] }
            tokio = "1.0"
        "#};

        assert_eq!(cargo_toml.to_string(), expected);
    }

    // TODO: unignore this test when we publish cyndra-rocket
    #[ignore]
    #[test]
    /// Makes sure that Rocket uses allow_prerelease flag when fetching the latest version
    fn test_get_latest_dependency_version_rocket() {
        let mut cargo_toml = cargo_toml_factory();
        let dependencies = cargo_toml["dependencies"].as_table_mut().unwrap();
        let manifest_path = PathBuf::new();
        let url = Url::parse("https://github.com/rust-lang/crates.io-index").unwrap();

        set_key_value_dependency_version(
            "cyndra-runtime",
            dependencies,
            &manifest_path,
            &url,
            true,
            mock_get_latest_dependency_version,
        );

        CyndraInitRocket.set_cargo_dependencies(
            dependencies,
            &manifest_path,
            &url,
            get_latest_dependency_version,
        );

        let version = dependencies["rocket"].as_str().unwrap();

        let expected = get_latest_dependency("rocket", true, &manifest_path, Some(&url))
            .expect("Could not query the latest version of rocket")
            .version()
            .expect("no rocket version found")
            .to_string();

        assert_eq!(version, expected);
    }
}

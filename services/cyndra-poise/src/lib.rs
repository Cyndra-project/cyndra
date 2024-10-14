//! Cyndra service integration for the Poise discord bot framework.
//! ## Example
//! ```rust,no_run
//! use cyndra_runtime::Context as _;
//! use poise::serenity_prelude as serenity;
//! use cyndra_secrets::SecretStore;
//! use cyndra_poise::CyndraPoise;
//!
//! struct Data {} // User data, which is stored and accessible in all command invocations
//! type Error = Box<dyn std::error::Error + Send + Sync>;
//! type Context<'a> = poise::Context<'a, Data, Error>;
//!
//! /// Responds with "world!"
//! #[poise::command(slash_command)]
//! async fn hello(ctx: Context<'_>) -> Result<(), Error> {
//!     ctx.say("world!").await?;
//!     Ok(())
//! }
//!
//! #[cyndra_runtime::main]
//! async fn poise(#[cyndra_secrets::Secrets] secret_store: SecretStore) -> CyndraPoise<Data, Error> {
//!     // Get the discord token set in `Secrets.toml`
//!     let discord_token = secret_store
//!         .get("DISCORD_TOKEN")
//!         .context("'DISCORD_TOKEN' was not found")?;
//!
//!     let framework = poise::Framework::builder()
//!         .options(poise::FrameworkOptions {
//!             commands: vec![hello()],
//!             ..Default::default()
//!         })
//!         .token(discord_token)
//!         .intents(serenity::GatewayIntents::non_privileged())
//!         .setup(|ctx, _ready, framework| {
//!             Box::pin(async move {
//!                 poise::builtins::register_globally(ctx, &framework.options().commands).await?;
//!                 Ok(Data {})
//!             })
//!         })
//!         .build()
//!         .await
//!         .map_err(cyndra_runtime::CustomError::new)?;
//!
//!     Ok(framework.into())
//! }
//! ```
use std::net::SocketAddr;
use std::sync::Arc;

/// A wrapper type for [poise::Framework] so we can implement [cyndra_runtime::Service] for it.
pub struct PoiseService<T, E>(pub Arc<poise::Framework<T, E>>);

#[cyndra_runtime::async_trait]
impl<T, E> cyndra_runtime::Service for PoiseService<T, E>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    async fn bind(mut self, _addr: SocketAddr) -> Result<(), cyndra_runtime::Error> {
        self.0
            .start()
            .await
            .map_err(cyndra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<T, E> From<Arc<poise::Framework<T, E>>> for PoiseService<T, E> {
    fn from(framework: Arc<poise::Framework<T, E>>) -> Self {
        Self(framework)
    }
}

/// The return type that should be returned from the [cyndra_runtime::main] function.
pub type CyndraPoise<T, E> = Result<PoiseService<T, E>, cyndra_runtime::Error>;

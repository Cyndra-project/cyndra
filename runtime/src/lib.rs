#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/logo-square-transparent.png",
    html_favicon_url = "https://raw.githubusercontent.com/cyndra-hq/cyndra/main/assets/favicon.ico"
)]

// Public API
pub use cyndra_codegen::main;
pub use cyndra_service::{
    CustomError, DbInput, DeploymentMetadata, Environment, Error, IntoResource, ResourceFactory,
    ResourceInputBuilder, SecretStore, Service,
};

// Useful re-exports
pub use async_trait::async_trait;
pub use tokio;

mod alpha;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn version() -> String {
    format!("{} {}", crate::NAME, crate::VERSION)
}

// Not part of public API
#[doc(hidden)]
pub mod __internals {
    // Internals used by the codegen
    pub use crate::alpha::{start, Alpha};

    // Dependencies required by the codegen
    pub use anyhow::Context;
    #[cfg(feature = "setup-tracing")]
    pub use colored;
    pub use serde_json;
    pub use strfmt::strfmt;
    #[cfg(feature = "setup-tracing")]
    pub use tracing_subscriber;
}

pub use plugins::*;
/// Built-in plugins
mod plugins {
    use crate::async_trait;
    use cyndra_service::{
        resource::{ProvisionResourceRequest, CyndraResourceOutput, Type},
        DeploymentMetadata, Error, ResourceFactory, ResourceInputBuilder, SecretStore,
    };

    /// ## Cyndra Metadata
    ///
    /// Plugin for getting various metadata at runtime.
    ///
    /// ### Usage
    ///
    /// ```rust,ignore
    /// #[cyndra_runtime::main]
    /// async fn main(
    ///     #[cyndra_runtime::Metadata] metadata: DeploymentMetadata,
    /// ) -> __ { ... }
    #[derive(Default)]
    pub struct Metadata;

    #[async_trait]
    impl ResourceInputBuilder for Metadata {
        type Input = DeploymentMetadata;
        type Output = DeploymentMetadata;

        async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, Error> {
            Ok(factory.get_metadata())
        }
    }

    /// ## Cyndra Secrets
    ///
    /// Plugin for getting secrets in your [Cyndra](https://www.cyndra.rs) service.
    ///
    /// ### Usage
    ///
    /// Add a `Secrets.toml` file to the root of your crate with the secrets you'd like to store.
    /// Make sure to add `Secrets*.toml` to `.gitignore` to omit your secrets from version control.
    ///
    /// Next, add `#[cyndra_runtime::Secrets] secrets: SecretStore` as a parameter to your `cyndra_service::main` function.
    /// `SecretStore::get` can now be called to retrieve your API keys and other secrets at runtime.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// #[cyndra_runtime::main]
    /// async fn main(
    ///     #[cyndra_runtime::Secrets] secrets: SecretStore
    /// ) -> CyndraAxum {
    ///     // get secret defined in `Secrets.toml` file.
    ///     let secret = secrets.get("MY_API_KEY").unwrap();
    ///
    ///     let router = Router::new()
    ///         .route("/", || async move { format!("My secret is: {}", secret) });
    ///
    ///     Ok(router.into())
    /// }
    /// ```
    #[derive(Default)]
    pub struct Secrets;

    #[async_trait]
    impl ResourceInputBuilder for Secrets {
        type Input = ProvisionResourceRequest;
        type Output = CyndraResourceOutput<SecretStore>;

        async fn build(self, _factory: &ResourceFactory) -> Result<Self::Input, Error> {
            Ok(ProvisionResourceRequest::new(
                Type::Secrets,
                serde_json::Value::Null,
                serde_json::Value::Null,
            ))
        }
    }
}

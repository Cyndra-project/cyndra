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

use async_trait::async_trait;
pub use cyndra_service::{DeploymentMetadata as Metadata, Environment, SecretStore};
use cyndra_service::{Error, ResourceFactory, ResourceInputBuilder};

#[derive(Default)]
pub struct CyndraMetadata;

#[async_trait]
impl ResourceInputBuilder for CyndraMetadata {
    type Input = Metadata;
    type Output = Metadata;

    async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, crate::Error> {
        Ok(factory.get_metadata())
    }
}

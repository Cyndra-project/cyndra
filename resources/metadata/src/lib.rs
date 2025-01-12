use async_trait::async_trait;
use cyndra_service::{error::Error, resource::Type, Factory, ResourceBuilder};
pub use cyndra_service::{DeploymentMetadata as Metadata, Environment};

#[derive(Default)]
pub struct CyndraMetadata;

#[async_trait]
impl ResourceBuilder for CyndraMetadata {
    const TYPE: Type = Type::Custom;
    type Config = ();
    type Output = Metadata;

    fn config(&self) -> &Self::Config {
        &()
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        Ok(factory.get_metadata())
    }
}

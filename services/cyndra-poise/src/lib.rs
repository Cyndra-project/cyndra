#![doc = include_str!("../README.md")]
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
            .start_autosharded()
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

#[doc = include_str!("../README.md")]
pub type CyndraPoise<T, E> = Result<PoiseService<T, E>, cyndra_runtime::Error>;

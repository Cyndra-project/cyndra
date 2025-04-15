#![doc = include_str!("../README.md")]
use cyndra_runtime::{CustomError, Error};
use std::net::SocketAddr;

#[cfg(feature = "axum")]
pub use axum;
#[cfg(feature = "axum-0-7")]
pub use axum_0_7 as axum;

#[cfg(feature = "axum")]
use axum::Router;
#[cfg(feature = "axum-0-7")]
use axum_0_7::Router;

/// A wrapper type for [axum::Router] so we can implement [cyndra_runtime::Service] for it.
pub struct AxumService(pub Router);

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for AxumService {
    /// Takes the router that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        #[cfg(feature = "axum")]
        axum::serve(
            cyndra_runtime::tokio::net::TcpListener::bind(addr)
                .await
                .map_err(CustomError::new)?,
            self.0,
        )
        .await
        .map_err(CustomError::new)?;
        #[cfg(feature = "axum-0-7")]
        axum_0_7::serve(
            cyndra_runtime::tokio::net::TcpListener::bind(addr)
                .await
                .map_err(CustomError::new)?,
            self.0,
        )
        .await
        .map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<Router> for AxumService {
    fn from(router: Router) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type CyndraAxum = Result<AxumService, Error>;

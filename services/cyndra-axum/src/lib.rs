//! Cyndra service integration for the Axum web framework.
//! ## Example
//! ```rust,no_run
//! use axum::{routing::get, Router};
//!
//! async fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn axum() -> cyndra_axum::CyndraAxum {
//!     let router = Router::new().route("/hello", get(hello_world));
//!
//!     Ok(router.into())
//! }
//! ```
use axum::routing::get;
use cyndra_runtime::{CustomError, Error};
use std::net::SocketAddr;

/// A wrapper type for [axum::Router] so we can implement [cyndra_runtime::Service] for it.
#[derive(Clone)]
pub struct AxumService(pub axum::Router);

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for AxumService {
    /// Takes the router that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        self.0 = self.0.route("/_cyndra/healthz", get(|| async {}));

        axum::Server::bind(&addr)
            .serve(self.0.into_make_service())
            .await
            .map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<axum::Router> for AxumService {
    fn from(router: axum::Router) -> Self {
        Self(router)
    }
}
/// The return type that should be returned from the [cyndra_runtime::main] function.
pub type CyndraAxum = Result<AxumService, Error>;

//! Cyndra service integration for the Tide web framework.
//! ## Example
//! ```rust,no_run
//! #[cyndra_runtime::main]
//! async fn tide() -> cyndra_tide::CyndraTide<()> {
//!     let mut app = tide::new();
//!     app.with(tide::log::LogMiddleware::new());
//!
//!     app.at("/hello").get(|_| async { Ok("Hello, world!") });
//!
//!     Ok(app.into())
//! }
//! ```
use cyndra_runtime::{CustomError, Error};
use std::net::SocketAddr;

/// A wrapper type for [tide::Server<T] so we can implement [cyndra_runtime::Service] for it.
pub struct TideService<T>(pub tide::Server<T>);

#[cyndra_runtime::async_trait]
impl<T> cyndra_runtime::Service for TideService<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Takes the router that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        self.0
            .at("/healthz")
            .get(|_| async { Ok(tide::StatusCode::Ok) });
        self.0.listen(addr).await.map_err(CustomError::new)?;
        Ok(())
    }
}

impl<T> From<tide::Server<T>> for TideService<T> {
    fn from(router: tide::Server<T>) -> Self {
        Self(router)
    }
}
/// The return type that should be returned from the [cyndra_runtime::main] function.
pub type CyndraTide<T> = Result<TideService<T>, Error>;

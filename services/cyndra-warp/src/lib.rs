//! Cyndra service integration for the Warp web framework.
//!
//! ## Example
//!
//! ```rust,no_run
//! use warp::Filter;
//! use warp::Reply;
//!
//! #[cyndra_runtime::main]
//! async fn warp() -> cyndra_warp::CyndraWarp<(impl Reply,)> {
//!     let route = warp::any().map(|| "Hello, World!");
//!     Ok(route.boxed().into())
//! }
//! ```
use cyndra_runtime::Error;
use std::net::SocketAddr;
use std::ops::Deref;

/// A wrapper type for [warp::Filter] so we can implement [cyndra_runtime::Service] for it.
pub struct WarpService<T>(pub T);

#[cyndra_runtime::async_trait]
impl<T> cyndra_runtime::Service for WarpService<T>
where
    T: Send + Sync + Clone + 'static + warp::Filter,
    T::Extract: warp::reply::Reply,
{
    /// Takes the router that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        warp::serve((*self).clone()).run(addr).await;
        Ok(())
    }
}

impl<T> From<T> for WarpService<T>
where
    T: Send + Sync + Clone + 'static + warp::Filter,
    T::Extract: warp::reply::Reply,
{
    fn from(router: T) -> Self {
        Self(router)
    }
}

impl<T> Deref for WarpService<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The return type of the [cyndra_runtime::main] function for the Warp service.
///
/// ## Example
///
/// ```rust,no_run
/// use cyndra_warp::CyndraWarp;
/// use warp::Filter;
/// use warp::Reply;
///
/// #[cyndra_runtime::main]
/// async fn warp() -> CyndraWarp<(impl Reply,)> {
///     let route = warp::any().map(|| "Hello, World");
///     Ok(route.boxed().into())
/// }
/// ```
pub type CyndraWarp<T> = Result<WarpService<warp::filters::BoxedFilter<T>>, Error>;

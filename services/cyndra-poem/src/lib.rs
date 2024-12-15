//! Cyndra service integration for the Poem web framework.
//! ## Example
//! ```rust,no_run
//! use poem::{get, handler, Route};
//! use cyndra_poem::CyndraPoem;
//!
//! #[handler]
//! fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn poem() -> CyndraPoem<impl poem::Endpoint> {
//!     let app = Route::new().at("/", get(hello_world));
//!
//!     Ok(app.into())
//! }
//!
//! ```

/// A wrapper type for [poem::Endpoint] so we can implement [cyndra_runtime::Service] for it.
pub struct PoemService<T>(pub T);

#[cyndra_runtime::async_trait]
impl<T> cyndra_runtime::Service for PoemService<T>
where
    T: poem::Endpoint + Send + 'static,
{
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        poem::Server::new(poem::listener::TcpListener::bind(addr))
            .run(self.0)
            .await
            .map_err(cyndra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<T> From<T> for PoemService<T>
where
    T: poem::Endpoint + Send + 'static,
{
    fn from(router: T) -> Self {
        Self(router)
    }
}

/// Return type from the `[cyndra_runtime::main]` macro for a Poem-based service.
///
/// # Example
///
/// ```rust,no_run
/// use poem::{get, handler, Route};
/// use cyndra_poem::CyndraPoem;
/// #[handler]
/// fn hello_world() -> &'static str {
///     "Hello, world!"
/// }
///
/// #[cyndra_runtime::main]
/// async fn poem() -> CyndraPoem<impl poem::Endpoint> {
///     let app = Route::new().at("/", get(hello_world));
///
///     Ok(app.into())
/// }
/// ```
pub type CyndraPoem<T> = Result<PoemService<T>, cyndra_runtime::Error>;

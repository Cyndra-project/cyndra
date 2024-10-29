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
//!     let app = Route::new().at("/hello", get(hello_world));
//!
//!     Ok(app.into())
//! }
//!
//! ```

/// A wrapper type for [poem::Endpoint] so we can implement [cyndra_runtime::Service] for it.
pub struct PoemService<T>(pub T);

#[poem::handler]
fn healthz() -> poem::http::StatusCode {
    poem::http::StatusCode::OK
}

#[cyndra_runtime::async_trait]
impl<T> cyndra_runtime::Service for PoemService<T>
where
    T: poem::Endpoint + Send + 'static,
{
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        let app = poem::Route::new()
            .at("/healthz", poem::get(healthz))
            .nest("/", self.0);

        poem::Server::new(poem::listener::TcpListener::bind(addr))
            .run(app)
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

/// The return type that should be returned from the [cyndra_runtime::main] function.
pub type CyndraPoem<T> = Result<PoemService<T>, cyndra_runtime::Error>;

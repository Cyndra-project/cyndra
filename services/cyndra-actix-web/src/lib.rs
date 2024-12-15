//! Cyndra service integration for the Actix Web framework.
//!
//! ## Example
//!
//! ```rust,no_run
//! use actix_web::{get, web::ServiceConfig};
//! use cyndra_actix_web::CyndraActixWeb;
//!
//! #[get("/")]
//! async fn hello_world() -> &'static str {
//!     "Hello World!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn actix_web() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
//!     let config = move |cfg: &mut ServiceConfig| {
//!         cfg.service(hello_world);
//!     };
//!
//!     Ok(config.into())
//! }
//! ```

use std::net::SocketAddr;

/// A wrapper type for a closure that returns an [actix_web::web::ServiceConfig] so we can implement
/// [cyndra_runtime::Service] for it.
#[derive(Clone)]
pub struct ActixWebService<F>(pub F);

#[cyndra_runtime::async_trait]
impl<F> cyndra_runtime::Service for ActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    async fn bind(mut self, addr: SocketAddr) -> Result<(), cyndra_runtime::Error> {
        // Start a worker for each cpu, but no more than 4.
        let worker_count = num_cpus::get().min(4);

        let server =
            actix_web::HttpServer::new(move || actix_web::App::new().configure(self.0.clone()))
                .workers(worker_count)
                .bind(addr)?
                .run();

        server.await.map_err(cyndra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<F> From<F> for ActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    fn from(service_config: F) -> Self {
        Self(service_config)
    }
}

/// Return type from the `[cyndra_runtime::main]` macro for an Actix-based service.
///
/// # Example
/// ```rust,no_run
/// # use cyndra_actix_web::CyndraActixWeb;
/// # use actix_web::web::ServiceConfig;
///
/// #[cyndra_runtime::main]
/// async fn example_service() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
///     let config = move |_cfg: &mut ServiceConfig| {};
///     Ok(config.into())
/// }
/// ```
pub type CyndraActixWeb<F> = Result<ActixWebService<F>, cyndra_runtime::Error>;

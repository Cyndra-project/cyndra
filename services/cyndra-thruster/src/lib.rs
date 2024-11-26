//! Cyndra service integration for the Thruster web framework.
//! ## Example
//! ```rust,no_run
//! use thruster::{
//!     context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest},
//!     m, middleware_fn, App, MiddlewareNext, MiddlewareResult,
//! };
//!
//! #[middleware_fn]
//! async fn hello(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
//!     context.body("Hello, World!");
//!     Ok(context)
//! }
//!
//! #[cyndra_runtime::main]
//! async fn thruster() -> cyndra_thruster::CyndraThruster {
//!     let app = App::<HyperRequest, Ctx, ()>::create(generate_context, ()).get("/", m![hello]);
//!
//!     Ok(app.into())
//! }
//! ```
use cyndra_runtime::Error;
use std::net::SocketAddr;
use thruster::{
    context::basic_hyper_context::{BasicHyperContext as Ctx, HyperRequest},
    m, middleware_fn, App, HyperServer, MiddlewareNext, MiddlewareResult, ThrusterServer,
};

#[middleware_fn]
async fn healthz(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    Ok(context)
}

/// A wrapper type for [thruster::ThrusterServer] so we can implement [cyndra_runtime::Service] for it.
pub struct ThrusterService(pub App<HyperRequest, Ctx, ()>);

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for ThrusterService {
    /// Takes the server that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let server = HyperServer::new(self.0.get("/_cyndra/healthz", m![healthz]));

        server.build(&addr.ip().to_string(), addr.port()).await;

        Ok(())
    }
}

impl From<App<HyperRequest, Ctx, ()>> for ThrusterService {
    fn from(app: App<HyperRequest, Ctx, ()>) -> Self {
        Self(app)
    }
}
/// The return type that should be returned from the [cyndra_runtime::main] function.
pub type CyndraThruster = Result<ThrusterService, Error>;

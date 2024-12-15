//! Cyndra service integration for the Rocket web framework.
//! ## Example
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! # fn main() {
//! #[get("/")]
//! fn index() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[cyndra_runtime::main]
//! async fn rocket() -> cyndra_rocket::CyndraRocket {
//!     let rocket = rocket::build().mount("/", routes![index]);
//!
//!     Ok(rocket.into())
//! }
//! # }
//! ```
use std::net::SocketAddr;

/// A wrapper type for [rocket::Rocket<rocket::Build>] so we can implement [cyndra_runtime::Service] for it.
pub struct RocketService(pub rocket::Rocket<rocket::Build>);

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for RocketService {
    /// Takes the router that is returned by the user in their [cyndra_runtime::main] function
    /// and binds to an address passed in by cyndra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), cyndra_runtime::Error> {
        let shutdown = rocket::config::Shutdown {
            ctrlc: false,
            ..rocket::config::Shutdown::default()
        };

        let config = self
            .0
            .figment()
            .clone()
            .merge((rocket::Config::ADDRESS, addr.ip()))
            .merge((rocket::Config::PORT, addr.port()))
            .merge((rocket::Config::LOG_LEVEL, rocket::config::LogLevel::Off))
            .merge((rocket::Config::SHUTDOWN, shutdown));

        let _rocket = self
            .0
            .configure(config)
            .launch()
            .await
            .map_err(cyndra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl From<rocket::Rocket<rocket::Build>> for RocketService {
    fn from(router: rocket::Rocket<rocket::Build>) -> Self {
        Self(router)
    }
}

/// Return type from the `[cyndra_runtime::main]` macro for a Rocket-based service.
///
/// # Example
///
/// ```rust,no_run
/// use rocket::{routes, get};
/// use cyndra_rocket::CyndraRocket;
///
/// #[get("/")]
/// fn index() -> &'static str {
///     "Hello, world!"
/// }
///
/// #[cyndra_runtime::main]
/// async fn rocket() -> CyndraRocket {
///     let rocket = rocket::build().mount("/", routes![index]);
///
///     Ok(rocket.into())
/// }
/// ```
pub type CyndraRocket = Result<RocketService, cyndra_runtime::Error>;

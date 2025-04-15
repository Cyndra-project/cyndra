#![doc = include_str!("../README.md")]
use std::net::SocketAddr;

pub use rocket;

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

#[doc = include_str!("../README.md")]
pub type CyndraRocket = Result<RocketService, cyndra_runtime::Error>;

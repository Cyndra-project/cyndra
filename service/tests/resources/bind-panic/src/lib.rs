use async_trait::async_trait;

use cyndra_service::{IntoService, ServeHandle, Service};

#[macro_use]
extern crate cyndra_service;

#[derive(Default)]
struct Builder;

impl IntoService for Builder {
    type Service = MyService;

    fn into_service(self) -> Self::Service {
        MyService
    }
}

struct MyService;

#[async_trait]
impl Service for MyService {
    fn bind(
        &mut self,
        _: std::net::SocketAddr,
    ) -> Result<ServeHandle, cyndra_service::error::Error> {
        panic!("panic in bind");
    }
}

declare_service!(Builder, Builder::default);

use cyndra_service::Service;

struct MyService;

#[cyndra_service::async_trait]
impl Service for MyService {
    async fn bind(
        mut self: Box<Self>,
        _: std::net::SocketAddr,
    ) -> Result<(), cyndra_service::Error> {
        panic!("panic in bind");
    }
}

#[cyndra_service::main]
async fn bind_panic() -> Result<MyService, cyndra_service::Error> {
    Ok(MyService)
}

use cyndra_service::Service;

struct MyService;

#[cyndra_service::async_trait]
impl Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), cyndra_service::Error> {
        Ok(())
    }
}

#[cyndra_service::main]
async fn main_panic() -> Result<MyService, cyndra_service::Error> {
    panic!("main panic")
}

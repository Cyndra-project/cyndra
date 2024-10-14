struct MyService;

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        Ok(())
    }
}

#[cyndra_runtime::main]
async fn main_panic() -> Result<MyService, cyndra_runtime::Error> {
    panic!("main panic")
}

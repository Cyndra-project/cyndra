struct MyService;

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        Ok(())
    }
}

#[derive(Default)]
struct Thing;

#[cyndra_runtime::async_trait]
impl cyndra_service::ResourceInputBuilder for Thing {
    type Input = ();
    type Output = ();

    async fn build(
        self,
        _factory: &cyndra_service::ResourceFactory,
    ) -> Result<Self::Input, cyndra_service::Error> {
        panic!("load panic");
    }
}

#[cyndra_runtime::main]
async fn load_panic(#[Thing] _a: ()) -> Result<MyService, cyndra_runtime::Error> {
    Ok(MyService)
}

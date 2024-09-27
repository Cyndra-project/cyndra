use tracing::debug;

cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        debug!("called hello()");
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        debug!("called goodbye()");
        "Goodbye, World!"
    }
}

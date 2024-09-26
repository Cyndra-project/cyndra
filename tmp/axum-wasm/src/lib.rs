cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

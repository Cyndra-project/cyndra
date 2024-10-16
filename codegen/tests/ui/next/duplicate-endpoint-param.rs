cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, method = get)]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(route = "/goodbye", route = "/goodbye")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, route = "/hello" extra = abundant)]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(method = get, route = "/goodbye", invalid)]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

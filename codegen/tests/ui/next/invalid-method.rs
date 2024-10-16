cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = pet, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(method =, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }
}

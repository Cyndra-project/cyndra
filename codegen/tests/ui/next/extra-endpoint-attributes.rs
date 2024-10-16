cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, route = "/hello")]
    #[cyndra_codegen::endpoint(method = post, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }
}

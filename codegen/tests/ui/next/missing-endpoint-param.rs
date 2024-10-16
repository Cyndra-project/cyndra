cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get)]
    async fn only_method_param() -> &'static str {
        "Hello, World!"
    }

    #[cyndra_codegen::endpoint(route = "/goodbye")]
    async fn only_route_param() -> &'static str {
        "Goodbye, World!"
    }

    #[cyndra_codegen::endpoint()]
    async fn no_params() -> &'static str {
        "Goodbye, World!"
    }
}

cyndra_codegen::app! {
    #[cyndra_codegen::endpoint(method = get, route = "/goodbye", invalid = bad)]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

cyndra_next::app! {
    #[cyndra_next::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        shared::hello()
    }
}

#[cyndra_runtime::main]
async fn axum() -> cyndra_axum::CyndraAxum {
    let router = axum::Router::new();

    Ok(router.into())
}

#[cyndra_service::main]
async fn axum() -> cyndra_service::CyndraAxum {
    let router = axum::Router::new();

    Ok(router)
}

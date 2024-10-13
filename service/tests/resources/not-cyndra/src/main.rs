// This service cannot be hosted on cyndra since it is missing the runtime the cyndra main macro would have added!!!
async fn axum() -> cyndra_service::CyndraAxum {
    let router = axum::Router::new();

    Ok(router)
}

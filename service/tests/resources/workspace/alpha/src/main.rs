use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    shared::hello()
}

#[cyndra_runtime::main]
async fn axum() -> cyndra_axum::CyndraAxum {
    let router = Router::new().route("/hello", get(hello_world));

    Ok(router.into())
}

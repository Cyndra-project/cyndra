use poem::{get, handler, Route};

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_service::main]
async fn main() -> cyndra_service::CyndraPoem<impl poem::Endpoint> {
    let app = Route::new().at("/hello", get(hello_world));

    Ok(app)
}

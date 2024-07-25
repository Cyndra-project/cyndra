
#[cyndra_service::main]
async fn tide() -> Result<tide::Server<()>, cyndra_service::Error> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/hello")
        .get(|_| async { Ok("Hello, world!") });

    Ok(app)
}
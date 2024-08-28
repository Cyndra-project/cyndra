use salvo::prelude::*;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello, world!"));
}

#[cyndra_service::main]
async fn salvo() -> cyndra_service::CyndraSalvo {
    let router = Router::with_path("hello").get(hello_world);

    Ok(router)
}

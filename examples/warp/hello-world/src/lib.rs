use warp::Filter;
use warp::Reply;

#[cyndra_service::main]
async fn warp() -> cyndra_service::CyndraWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Hello, World");
    Ok(route.boxed())
}

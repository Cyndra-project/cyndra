// This will fail to compile since it's a library.

#[cyndra_service::main]
async fn rocket() -> cyndra_service::CyndraRocket {
    let rocket = rocket::build();
    Ok(rocket)
}

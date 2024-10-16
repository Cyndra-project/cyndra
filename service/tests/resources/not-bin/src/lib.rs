// This will fail to compile since it's a library.

#[cyndra_runtime::main]
async fn rocket() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build();
    Ok(rocket.into())
}

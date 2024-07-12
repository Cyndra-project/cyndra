#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_service::main]
async fn rocket() -> Result<Rocket<Build>, cyndra_service::Error> {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket)
}

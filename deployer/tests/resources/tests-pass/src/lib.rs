#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_service::main]
async fn rocket() -> cyndra_service::CyndraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);
    Ok(rocket)
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_fails() {
        assert_eq!(super::index(), "Hello, world!");
    }
}

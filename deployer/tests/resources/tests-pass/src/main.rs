#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn rocket() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);
    Ok(rocket.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_passes() {
        assert_eq!(super::index(), "Hello, world!");
    }
}

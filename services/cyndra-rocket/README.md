## Cyndra service integration for the Rocket web framework

### Example

```rust,no_run
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn rocket() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
```

#[macro_use]
extern crate rocket;

use anyhow::anyhow;
use rocket::response::status::BadRequest;
use rocket::State;
use cyndra_secrets::SecretStore;

#[get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    Ok(state.secret.clone())
}

struct MyState {
    secret: String,
}

#[cyndra_service::main]
async fn rocket(
    #[cyndra_secrets::Secrets] secret_store: SecretStore,
) -> cyndra_service::CyndraRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = if let Some(secret) = secret_store.get("MY_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let state = MyState { secret };
    let rocket = rocket::build().mount("/", routes![secret]).manage(state);

    Ok(rocket)
}

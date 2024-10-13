// The few line below is what we should now codegen for legacy
#[tokio::main]
async fn main() {
    cyndra_runtime::start(loader).await;
}

async fn loader<S: cyndra_common::storage_manager::StorageManager>(
    mut factory: cyndra_runtime::ProvisionerFactory<S>,
    _logger: cyndra_runtime::Logger,
) -> cyndra_service::CyndraRocket {
    use cyndra_service::ResourceBuilder;

    let secrets = cyndra_secrets::Secrets::new().build(&mut factory).await?;

    rocket(secrets).await
}

// Everything below this is the usual code a user will write
use anyhow::anyhow;
use rocket::response::status::BadRequest;
use rocket::State;
use cyndra_secrets::SecretStore;

#[rocket::get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    Ok(state.secret.clone())
}

struct MyState {
    secret: String,
}

// #[cyndra_service::main]
pub async fn rocket(
    // #[cyndra_secrets::Secrets] secret_store: SecretStore,
    secret_store: SecretStore,
) -> cyndra_service::CyndraRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = if let Some(secret) = secret_store.get("MY_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let state = MyState { secret };
    let rocket = rocket::build()
        .mount("/", rocket::routes![secret])
        .manage(state);

    Ok(rocket)
}

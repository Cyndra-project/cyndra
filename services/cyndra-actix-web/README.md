## Cyndra service integration for the Actix Web framework

### Example

```rust,no_run
use actix_web::{get, web::ServiceConfig};
use cyndra_actix_web::CyndraActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[cyndra_runtime::main]
async fn actix_web() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
```

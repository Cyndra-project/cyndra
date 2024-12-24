## Cyndra service integration for the Poem web framework.

### Example

```rust,no_run
use poem::{get, handler, Route};
use cyndra_poem::CyndraPoem;

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn poem() -> CyndraPoem<impl poem::Endpoint> {
    let app = Route::new().at("/", get(hello_world));

    Ok(app.into())
}
```

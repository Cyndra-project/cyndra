## Cyndra service integration for the Tide web framework

### Example

```rust,no_run
#[cyndra_runtime::main]
async fn tide() -> cyndra_tide::CyndraTide<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });

    Ok(app.into())
}
```

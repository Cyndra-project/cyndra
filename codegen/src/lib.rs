mod cyndra_main;

/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// ## Without cyndra managed resources
/// The simplest usage is when your service does not require any cyndra managed resources, so you only need to return a cyndra supported service:
///
/// ```rust,no_run
/// use cyndra_rocket::CyndraRocket;
///
/// #[cyndra_runtime::main]
/// async fn rocket() -> CyndraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket.into())
/// }
/// ```
///
/// ## Cyndra supported services
/// The following types can be returned from a `#[cyndra_runtime::main]` function and enjoy first class service support in cyndra.
///
/// | Return type       | Crate                                                          | Service                                                                 | Example                                                                                 |
/// | ----------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
/// | `CyndraActixWeb` | [cyndra-actix-web](https://crates.io/crates/cyndra-actix-web)| [actix-web](https://docs.rs/actix-web)                                  | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/actix-web/hello-world)|
/// | `CyndraAxum`     | [cyndra-axum](https://crates.io/crates/cyndra-axum)          | [axum](https://docs.rs/axum)                                            | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/axum/hello-world)     |
/// | `CyndraPoem`     | [cyndra-poem](https://crates.io/crates/cyndra-poem)          | [poem](https://docs.rs/poem)                                            | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/poem/hello-world)     |
/// | `CyndraRocket`   | [cyndra-rocket](https://crates.io/crates/cyndra-rocket)      | [rocket](https://docs.rs/rocket)                                        | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/rocket/hello-world)   |
/// | `CyndraSalvo`    | [cyndra-salvo](https://crates.io/crates/cyndra-salvo)        | [salvo](https://docs.rs/salvo)                                          | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/salvo/hello-world)    |
/// | `CyndraSerenity` | [cyndra-serenity](https://crates.io/crates/cyndra-serenity)  | [serenity](https://docs.rs/serenity) and [poise](https://docs.rs/poise) | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/serenity/hello-world) |
/// | `CyndraThruster` | [cyndra-thruster](https://crates.io/crates/cyndra-thruster)  | [thruster](https://docs.rs/thruster)                                    | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/thruster/hello-world) |
/// | `CyndraTower`    | [cyndra-tower](https://crates.io/crates/cyndra-tower)        | [tower](https://docs.rs/tower)                                          | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/tower/hello-world)    |
/// | `CyndraTide`     | [cyndra-tide](https://crates.io/crates/cyndra-tide)          | [tide](https://docs.rs/tide)                                            | [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/tide/hello-world)     |
///
/// ## Getting cyndra managed resources
/// Cyndra is able to manage resource dependencies for you. These resources are passed in as inputs to your `#[cyndra_runtime::main]` function and are configured using attributes:
/// ```rust,no_run
/// use sqlx::PgPool;
/// use cyndra_rocket::CyndraRocket;
///
/// struct MyState(PgPool);
///
/// #[cyndra_runtime::main]
/// async fn rocket(#[cyndra_shared_db::Postgres] pool: PgPool) -> CyndraRocket {
///     let state = MyState(pool);
///     let rocket = rocket::build().manage(state);
///
///     Ok(rocket.into())
/// }
/// ```
///
/// More [cyndra managed resources can be found here](https://github.com/cyndra-hq/cyndra/tree/main/resources)
#[proc_macro_error2::proc_macro_error]
#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    cyndra_main::tokens(attr, item)
}

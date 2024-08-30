# Cyndra Secrets
This plugin manages secrets on [cyndra](https://www.cyndra.rs).

## Usage
Add `cyndra-secrets` to the dependencies for your service. Also add a dependency which will give you a `PgPool` like [cyndra-shared-db](https://github.com/cyndra-hq/cyndra/tree/main/resources/shared-db)

[`SecretStore::get_secret`] can now be called on any instance of this pool to retrieve stored secrets.

An example using the Rocket framework can be found on [GitHub](https://github.com/cyndra-hq/cyndra/tree/main/examples/rocket/postgres)


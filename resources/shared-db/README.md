# Cyndra Shared Databases

This plugin manages databases that are shared with other services on [cyndra](https://www.cyndra.rs).

## Usage

Add `cyndra-shared-db` to the dependencies for your service. Every type of shareable database is behind the following feature flag and attribute path (`*-rustls` uses rustls for TLS, the default uses native-tls).

| Engine   | Feature flags                  | Attribute path              |
|----------|--------------------------------|-----------------------------|
| Postgres | `postgres` / `postgres-rustls` | cyndra_shared_db::Postgres |
| MongoDB  | `mongodb`                      | cyndra_shared_db::MongoDb  |

An example using the Rocket framework can be found on [GitHub](https://github.com/cyndra-hq/cyndra-examples/tree/main/rocket/postgres)

### Postgres

This resource has the following options

| Option    | Type | Description                                                                                                    |
|-----------|------|----------------------------------------------------------------------------------------------------------------|
| local_uri | &str | Don't spin a local docker instance of Postgres, but rather connect to this URI instead for `cargo cyndra run` |

### MongoDB

This resource has the following options

| Option    | Type | Description                                                                                                   |
|-----------|------|---------------------------------------------------------------------------------------------------------------|
| local_uri | &str | Don't spin a local docker instance of MongoDB, but rather connect to this URI instead for `cargo cyndra run` |

# Cyndra Shared Databases
This plugin manages databases that are shared with other services on [cyndra](https://www.cyndra.rs).

## Usage
Add `cyndra-shared-db` to the dependencies for your service. Every type of shareable database is behind the following feature flag and attribute path

| Engine   | Feature flag | Attribute path              |
|----------|--------------|-----------------------------|
| Postgres | postgres     | cyndra_shared_db::Postgres |
| MongoDB  | mongodb      | cyndra_shared_db::MongoDb  |

An example using the Rocket framework can be found on [GitHub](https://github.com/cyndra-hq/examples/tree/main/rocket/postgres)


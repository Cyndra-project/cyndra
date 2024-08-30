# Cyndra AWS RDS
This plugin provisions databases on AWS RDS using [cyndra](https://www.cyndra.rs). The following three engines are supported:
- Postgres
- MySql
- MariaDB

## Usage
Add `cyndra-aws-rds` to the dependencies for your service. Every engine is behind the following feature flags and attribute paths:

| Engine   | Feature flag | Attribute path            |
|----------|--------------|---------------------------|
| Postgres | postgres     | cyndra_aws_rds::Postgres |
| MySql    | mysql        | cyndra_aws_rds::MySql    |
| MariaDB  | mariadb      | cyndra_aws_rds::MariaDB  |

An example using the Tide framework can be found on [GitHub](https://github.com/cyndra-hq/cyndra/tree/main/examples/tide/postgres)


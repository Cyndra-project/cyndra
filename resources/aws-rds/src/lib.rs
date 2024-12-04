#![doc = include_str!("../README.md")]

macro_rules! aws_engine {
    ($feature:expr, $pool_path:path, $options_path:path, $struct_ident:ident) => {
        paste::paste! {
            #[derive(serde::Serialize)]
            #[cfg(feature = $feature)]
            #[doc = "A resource connected to an AWS RDS " $struct_ident " instance"]
            pub struct $struct_ident{
                config: cyndra_service::DbInput,
            }

            #[cfg(feature = $feature)]
            #[doc = "Gets a `sqlx::Pool` connected to an AWS RDS " $struct_ident " instance"]
            #[async_trait::async_trait]
            impl cyndra_service::ResourceBuilder<$pool_path> for $struct_ident {
                const TYPE: cyndra_service::Type = cyndra_service::Type::Database(
                    cyndra_service::database::Type::AwsRds(
                        cyndra_service::database::AwsRdsEngine::$struct_ident
                    )
                );

                type Config = cyndra_service::DbInput;
                type Output = cyndra_service::DbOutput;

                fn new() -> Self {
                    Self { config: Default::default() }
                }

                fn config(&self) -> &Self::Config {
                    &self.config
                }

                async fn output(self, factory: &mut dyn cyndra_service::Factory) -> Result<Self::Output, cyndra_service::Error> {
                    let info = match factory.get_metadata().env {
                        cyndra_service::Environment::Deployment => cyndra_service::DbOutput::Info(
                            factory
                                .get_db_connection(cyndra_service::database::Type::AwsRds(cyndra_service::database::AwsRdsEngine::$struct_ident))
                                .await?
                        ),
                        cyndra_service::Environment::Local => {
                            if let Some(local_uri) = self.config.local_uri {
                                cyndra_service::DbOutput::Local(local_uri)
                            } else {
                                cyndra_service::DbOutput::Info(
                                    factory
                                        .get_db_connection(cyndra_service::database::Type::AwsRds(cyndra_service::database::AwsRdsEngine::$struct_ident))
                                        .await?
                                )
                            }
                        }
                    };

                    Ok(info)
                }

                async fn build(build_data: &Self::Output) -> Result<$pool_path, cyndra_service::Error> {
                    let connection_string = match build_data {
                        cyndra_service::DbOutput::Local(local_uri) => local_uri.clone(),
                        cyndra_service::DbOutput::Info(info) => info.connection_string_private(),
                    };

                    let pool = $options_path::new()
                        .min_connections(1)
                        .max_connections(5)
                        .connect(&connection_string)
                        .await
                        .map_err(cyndra_service::error::CustomError::new)?;

                    Ok(pool)
                }
            }

            #[cfg(feature = $feature)]
            impl $struct_ident {
                /// Use a custom connection string for local runs
                pub fn local_uri(mut self, local_uri: &str) -> Self {
                    self.config.local_uri = Some(local_uri.to_string());

                    self
                }
            }
        }
    };
}

aws_engine!(
    "postgres",
    sqlx::PgPool,
    sqlx::postgres::PgPoolOptions,
    Postgres
);

aws_engine!(
    "mysql",
    sqlx::MySqlPool,
    sqlx::mysql::MySqlPoolOptions,
    MySql
);

aws_engine!(
    "mariadb",
    sqlx::MySqlPool,
    sqlx::mysql::MySqlPoolOptions,
    MariaDB
);

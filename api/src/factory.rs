use async_trait::async_trait;
use cyndra_common::DatabaseReadyInfo;
use cyndra_service::Factory;

use crate::database;

pub(crate) struct CyndraFactory {
    database: database::State,
}

impl CyndraFactory {
    pub(crate) fn new(database: database::State) -> Self {
        Self { database }
    }
}

impl CyndraFactory {
    pub(crate) fn to_database_info(&self) -> Option<DatabaseReadyInfo> {
        self.database.to_info()
    }
}

#[async_trait]
impl Factory for CyndraFactory {
    async fn get_sql_connection_string(&mut self) -> Result<String, cyndra_service::Error> {
        let conn_str = self
            .database
            .request()
            .await
            .map_err(cyndra_service::error::CustomError::new)?
            .connection_string("localhost");
        debug!("giving a sql connection string: {}", conn_str);
        Ok(conn_str)
    }
}

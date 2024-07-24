use async_trait::async_trait;
use cyndra_service::Factory;

use crate::database;

pub(crate) struct CyndraFactory<'a> {
    database: &'a mut database::State,
}

impl<'a> CyndraFactory<'a> {
    pub(crate) fn new(database: &'a mut database::State) -> Self {
        Self { database }
    }
}

#[async_trait]
impl Factory for CyndraFactory<'_> {
    async fn get_sql_connection_string(&mut self) -> Result<String, cyndra_service::Error> {
        let conn_str = self.database.request().connection_string("localhost");
        debug!("giving a sql connection string: {}", conn_str);
        Ok(conn_str)
    }
}

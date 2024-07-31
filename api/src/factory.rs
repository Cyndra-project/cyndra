use async_trait::async_trait;
use proto::provisioner::{provisioner_client::ProvisionerClient, DatabaseRequest};
use cyndra_common::{project::ProjectName, DatabaseReadyInfo};
use cyndra_service::Factory;
use tonic::{transport::Channel, Request};

pub(crate) struct CyndraFactory {
    project_name: ProjectName,
    provisioner_client: ProvisionerClient<Channel>,
    provisioner_address: String,
    info: Option<DatabaseReadyInfo>,
}

impl CyndraFactory {
    pub(crate) fn new(
        provisioner_client: ProvisionerClient<Channel>,
        provisioner_address: String,
        project_name: ProjectName,
    ) -> Self {
        Self {
            provisioner_client,
            provisioner_address,
            project_name,
            info: None,
        }
    }

    pub(crate) fn into_database_info(self) -> Option<DatabaseReadyInfo> {
        self.info
    }
}

#[async_trait]
impl Factory for CyndraFactory {
    async fn get_sql_connection_string(&mut self) -> Result<String, cyndra_service::Error> {
        if let Some(ref info) = self.info {
            return Ok(info.connection_string(&self.provisioner_address));
        }

        let request = Request::new(DatabaseRequest {
            project_name: self.project_name.to_string(),
        });

        let response = self
            .provisioner_client
            .provision_database(request)
            .await
            .map_err(cyndra_service::error::CustomError::new)?
            .into_inner();

        let info: DatabaseReadyInfo = response.into();
        let conn_str = info.connection_string(&self.provisioner_address);
        self.info = Some(info);

        debug!("giving a sql connection string: {}", conn_str);
        Ok(conn_str)
    }
}

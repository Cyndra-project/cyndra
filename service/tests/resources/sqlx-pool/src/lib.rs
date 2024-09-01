use cyndra_service::error::CustomError;
use cyndra_service::Service;
use sqlx::PgPool;

struct PoolService {
    pool: PgPool,
}

#[cyndra_service::main]
async fn init(
    #[cyndra_shared_db::Postgres] pool: PgPool,
) -> Result<PoolService, cyndra_service::Error> {
    Ok(PoolService { pool })
}

impl PoolService {
    async fn start(&self) -> Result<(), cyndra_service::error::CustomError> {
        let (rec,): (String,) = sqlx::query_as("SELECT 'Hello world'")
            .fetch_one(&self.pool)
            .await
            .map_err(CustomError::new)?;

        assert_eq!(rec, "Hello world");

        Ok(())
    }
}

#[cyndra_service::async_trait]
impl Service for PoolService {
    async fn bind(
        mut self: Box<Self>,
        _: std::net::SocketAddr,
    ) -> Result<(), cyndra_service::error::Error> {
        self.start().await?;

        Ok(())
    }
}

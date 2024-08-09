use std::time::Duration;

use cyndra_service::Service;
use tokio::time::sleep;

struct SleepService {
    duration: u64,
}

#[cyndra_service::main]
async fn simple() -> Result<SleepService, cyndra_service::Error> {
    Ok(SleepService { duration: 10 })
}

#[cyndra_service::async_trait]
impl Service for SleepService {
    async fn bind(
        mut self: Box<Self>,
        _: std::net::SocketAddr,
    ) -> Result<(), cyndra_service::error::Error> {
        let duration = Duration::from_secs(self.duration);

        sleep(duration).await;
        Ok(())
    }
}

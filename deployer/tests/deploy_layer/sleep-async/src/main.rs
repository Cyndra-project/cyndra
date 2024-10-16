use std::time::Duration;

use cyndra_runtime::Service;
use tokio::time::sleep;

struct SleepService {
    duration: u64,
}

#[cyndra_runtime::main]
async fn simple() -> Result<SleepService, cyndra_runtime::Error> {
    Ok(SleepService { duration: 4 })
}

#[cyndra_runtime::async_trait]
impl Service for SleepService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        let duration = Duration::from_secs(self.duration);

        sleep(duration).await;
        Ok(())
    }
}

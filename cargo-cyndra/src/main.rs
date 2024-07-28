use anyhow::Result;
use cargo_cyndra::{Args, Cyndra};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    Cyndra::new().run(Args::from_args()).await
}

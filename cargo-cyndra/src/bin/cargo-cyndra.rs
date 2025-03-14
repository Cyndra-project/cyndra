use anyhow::Result;
use cargo_cyndra::{parse_args, setup_tracing, Binary, Cyndra};

#[tokio::main]
async fn main() -> Result<()> {
    let (args, provided_path_to_init) = parse_args();

    setup_tracing(args.debug);

    Cyndra::new(Binary::CargoCyndra)?
        .run(args, provided_path_to_init)
        .await
}

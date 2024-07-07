use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    // Cargo passes in the subcommand name to the invoked executable. Use a
    // hidden, optional positional argument to deal with it.
    arg(structopt::clap::Arg::with_name("dummy")
        .possible_value("cyndra")
        .required(false)
        .hidden(true))
)]
pub enum Args {
    #[structopt(about = "deploy an cyndra project")]
    Deploy(DeployArgs),
    #[structopt(about = "view the status of an cyndra deployment")]
    Status,
    #[structopt(about = "view the status of an cyndra deployment")]
    Delete,
}

#[derive(StructOpt)]
pub struct DeployArgs {
    #[structopt(long, about = "allow dirty working directories to be packaged")]
    pub allow_dirty: bool,
}

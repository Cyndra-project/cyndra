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
    #[structopt(about = "view the status of an cyndra project")]
    Status,
    #[structopt(about = "delete the latest deployment for a cyndra project")]
    Delete,
    #[structopt(about = "create user credentials for the cyndra platform")]
    Auth(AuthArgs),
    #[structopt(about = "login to the cyndra platform")]
    Login(LoginArgs),
}

#[derive(StructOpt)]
pub struct LoginArgs {
    #[structopt(long, about = "api key for the cyndra platform")]
    pub api_key: Option<String>,
}

#[derive(StructOpt)]
pub struct AuthArgs {
    #[structopt(about = "the desired username for the cyndra platform")]
    pub username: String,
}

#[derive(StructOpt)]
pub struct DeployArgs {
    #[structopt(long, about = "allow dirty working directories to be packaged")]
    pub allow_dirty: bool,
}

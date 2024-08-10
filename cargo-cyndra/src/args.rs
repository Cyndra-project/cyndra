use std::{
    ffi::OsStr,
    fs::{canonicalize, create_dir_all},
    io::{self, ErrorKind},
    path::PathBuf,
};

use clap::Parser;
use cyndra_common::project::ProjectName;

#[derive(Parser)]
#[clap(
    // Cargo passes in the subcommand name to the invoked executable. Use a
    // hidden, optional positional argument to deal with it.
    arg(clap::Arg::with_name("dummy")
        .possible_value("cyndra")
        .required(false)
        .hidden(true))
)]
pub struct Args {
    /// run this command against the api at the supplied url
    /// (allows targeting a custom deployed instance for this command only)
    #[clap(long, env = "cyndra_API")]
    pub api_url: Option<String>,
    #[clap(flatten)]
    pub project_args: ProjectArgs,
    #[clap(subcommand)]
    pub cmd: Command,
}

// Common args for subcommands that deal with projects.
#[derive(Parser, Debug)]
pub struct ProjectArgs {
    /// Specify the working directory
    #[clap(
        global = true,
        long,
        parse(try_from_os_str = parse_path),
        default_value = ".",
    )]
    pub working_directory: PathBuf,
    /// Specify the name of the project (overrides crate name)
    #[clap(global = true, long)]
    pub name: Option<ProjectName>,
}

#[derive(Parser)]
pub enum Command {
    /// deploy a cyndra project
    Deploy(DeployArgs),
    /// create a new cyndra project
    Init(InitArgs),
    /// view the status of a cyndra project
    Status,
    /// view the logs of a cyndra project
    Logs,
    /// delete the latest deployment for a cyndra project
    Delete,
    /// create user credentials for the cyndra platform
    Auth(AuthArgs),
    /// login to the cyndra platform
    Login(LoginArgs),
    /// run a cyndra project locally
    Run(RunArgs),
}

#[derive(Parser)]
pub struct LoginArgs {
    /// api key for the cyndra platform
    #[clap(long)]
    pub api_key: Option<String>,
}

#[derive(Parser)]
pub struct AuthArgs {
    /// the desired username for the cyndra platform
    #[clap()]
    pub username: String,
}

#[derive(Parser)]
pub struct DeployArgs {
    /// allow dirty working directories to be packaged
    #[clap(long)]
    pub allow_dirty: bool,
    /// allows pre-deploy tests to be skipped
    #[clap(long)]
    pub no_test: bool,
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// port to start service on
    #[clap(long, default_value = "8000")]
    pub port: u16,
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Initialize with axum framework
    #[clap(long, conflicts_with_all = &["rocket", "tide", "tower"])]
    pub axum: bool,
    /// Initialize with actix-web framework
    #[clap(long, conflicts_with_all = &["axum", "tide", "tower"])]
    pub rocket: bool,
    /// Initialize with tide framework
    #[clap(long, conflicts_with_all = &["axum", "rocket", "tower"])]
    pub tide: bool,
    /// Initialize with tower framework
    #[clap(long, conflicts_with_all = &["axum", "rocket", "tide"])]
    pub tower: bool,
    /// Path to initialize a new cyndra project
    #[clap(
        parse(try_from_os_str = parse_init_path),
        default_value = ".",
    )]
    pub path: PathBuf,
}

// Helper function to parse and return the absolute path
fn parse_path(path: &OsStr) -> Result<PathBuf, io::Error> {
    canonicalize(path).map_err(|e| {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("could not turn {path:?} into a real path: {e}"),
        )
    })
}

// Helper function to parse, create if not exists, and return the absolute path
fn parse_init_path(path: &OsStr) -> Result<PathBuf, io::Error> {
    // Create the directory if does not exist
    create_dir_all(path)?;

    parse_path(path)
}

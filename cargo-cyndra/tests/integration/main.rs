mod builder;
mod init;
mod run;

use cargo_cyndra::{Command, ProjectArgs, Cyndra, CyndraArgs};
use std::path::Path;

/// Creates a CLI instance with some reasonable defaults set
async fn cyndra_command(cmd: Command, working_directory: &str) -> anyhow::Result<()> {
    let working_directory = Path::new(working_directory).to_path_buf();

    Cyndra::new(cargo_cyndra::Binary::Cyndra, None)
        .unwrap()
        .run(
            CyndraArgs {
                api_url: Some("http://cyndra.invalid:80".to_string()),
                admin: false,
                api_env: None,
                project_args: ProjectArgs {
                    working_directory,
                    name_or_id: None,
                },
                offline: false,
                debug: false,
                cmd,
            },
            false,
        )
        .await
}

#[tokio::test]
#[should_panic(expected = "failed to start `cargo metadata`: No such file or directory")]
async fn fails_if_working_directory_does_not_exist() {
    cyndra_command(
        Command::Logs(Default::default()),
        "/path_that_does_not_exist",
    )
    .await
    .unwrap();
}

#[tokio::test]
#[should_panic(expected = "could not find `Cargo.toml` in `/` or any parent directory")]
async fn fails_if_working_directory_not_part_of_cargo_workspace() {
    cyndra_command(Command::Logs(Default::default()), "/")
        .await
        .unwrap();
}

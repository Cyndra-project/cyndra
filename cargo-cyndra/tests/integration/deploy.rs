use std::path::Path;

use cargo_cyndra::{Args, Command, CommandOutcome, DeployArgs, ProjectArgs, Cyndra};
use reqwest::StatusCode;
use test_context::test_context;
use tokiotest_httpserver::{handler::HandlerBuilder, HttpTestContext};

/// creates a `cargo-cyndra` deploy instance with some reasonable defaults set.
async fn cargo_cyndra_deploy(path: &str, api_url: String) -> anyhow::Result<CommandOutcome> {
    let working_directory = Path::new(path).to_path_buf();

    Cyndra::new()
        .run(Args {
            api_url: Some(api_url),
            project_args: ProjectArgs {
                working_directory,
                name: None,
            },
            cmd: Command::Deploy(DeployArgs {
                allow_dirty: false,
                no_test: false,
            }),
        })
        .await
}

#[should_panic(
    expected = "Your cyndra-service version is outdated. Update your cyndra-service version to 1.2.5 and try to deploy again"
)]
#[test_context(HttpTestContext)]
#[tokio::test]
async fn deploy_when_version_is_outdated(ctx: &mut HttpTestContext) {
    ctx.add(
        HandlerBuilder::new("/test/version")
            .status_code(StatusCode::OK)
            .response("1.2.5".into())
            .build(),
    );
    let api_url = ctx.uri("/test").to_string();

    cargo_cyndra_deploy("../examples/rocket/hello-world", api_url)
        .await
        .unwrap();
}

#[should_panic(expected = "not an absolute path")]
#[test_context(HttpTestContext)]
#[tokio::test]
async fn deploy_when_version_is_valid(ctx: &mut HttpTestContext) {
    ctx.add(
        HandlerBuilder::new("/test/version")
            .status_code(StatusCode::OK)
            .response(cyndra_service::VERSION.into())
            .build(),
    );
    let api_url = ctx.uri("/test").to_string();

    cargo_cyndra_deploy("../examples/rocket/hello-world", api_url)
        .await
        .unwrap();
}

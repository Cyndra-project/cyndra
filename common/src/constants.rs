//! Shared constants used across Cyndra crates

/// Used by plugins for local file storage.
pub const STORAGE_DIRNAME: &str = ".cyndra-storage";

// URLs
pub const cyndra_API_URL: &str = "https://api.cyndra.dev";
pub const cyndra_CONSOLE_URL: &str = "https://console.cyndra.dev";

pub fn other_env_api_url(env: &str) -> String {
    format!("https://api.{env}.cyndra.dev")
}

pub const cyndra_INSTALL_DOCS_URL: &str = "https://docs.cyndra.dev/getting-started/installation";

pub const cyndra_GH_REPO_URL: &str = "https://github.com/cyndra-hq/cyndra";
pub const cyndra_GH_ISSUE_URL: &str = "https://github.com/cyndra-hq/cyndra/issues/new/choose";
pub const EXAMPLES_REPO: &str = "https://github.com/cyndra-hq/cyndra-examples";
pub const EXAMPLES_README: &str =
    "https://github.com/cyndra-hq/cyndra-examples#how-to-clone-run-and-deploy-an-example";
pub const EXAMPLES_TEMPLATES_TOML: &str =
    "https://raw.githubusercontent.com/cyndra-hq/cyndra-examples/main/templates.toml";

/// Crate name for checking cargo metadata
pub const RUNTIME_NAME: &str = "cyndra-runtime";

/// Current version field in `examples/templates.toml`
pub const TEMPLATES_SCHEMA_VERSION: u32 = 1;

pub mod headers {
    use http::HeaderName;

    pub static X_CARGO_cyndra_VERSION: HeaderName =
        HeaderName::from_static("x-cargo-cyndra-version");
}

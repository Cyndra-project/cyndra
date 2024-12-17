//
// Constants regarding the deployer environment and conventions
//
/// Where executables are moved to in order to persist across deploys, relative to workspace root
pub const EXECUTABLE_DIRNAME: &str = ".cyndra-executables";
/// Where general files will persist across deploys, relative to workspace root. Used by plugins.
pub const STORAGE_DIRNAME: &str = ".cyndra-storage";

// URLs
pub const API_URL_LOCAL: &str = "http://localhost:8001";
pub const API_URL_PRODUCTION: &str = "https://api.cyndra.rs";
#[cfg(debug_assertions)]
pub const API_URL_DEFAULT: &str = API_URL_LOCAL;
#[cfg(not(debug_assertions))]
pub const API_URL_DEFAULT: &str = API_URL_PRODUCTION;

pub const cyndra_STATUS_URL: &str = "https://status.cyndra.rs";
pub const cyndra_LOGIN_URL: &str = "https://console.cyndra.rs/new-project";
pub const cyndra_GH_ISSUE_URL: &str = "https://github.com/cyndra-hq/cyndra/issues/new/choose";
pub const cyndra_CLI_DOCS_URL: &str = "https://docs.cyndra.rs/getting-started/cyndra-commands";
pub const cyndra_IDLE_DOCS_URL: &str = "https://docs.cyndra.rs/getting-started/idle-projects";
pub const cyndra_EXAMPLES_README: &str =
    "https://github.com/cyndra-hq/cyndra-examples#how-to-clone-run-and-deploy-an-example";

// Crate names for checking cargo metadata
pub const NEXT_NAME: &str = "cyndra-next";
pub const RUNTIME_NAME: &str = "cyndra-runtime";

pub mod limits {
    pub const MAX_PROJECTS_DEFAULT: u32 = 3;
    pub const MAX_PROJECTS_EXTRA: u32 = 15;
}

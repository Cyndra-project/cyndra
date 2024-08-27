use std::path::Path;

use cyndra_common::version::get_cyndra_service_from_user_crate;

fn main() {
    let version = get_cyndra_service_from_user_crate(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .unwrap();
    println!(
        "cargo:rustc-env=cyndra_SERVICE_VERSION_REQ=^{}.{}",
        version.major, version.minor,
    );
}

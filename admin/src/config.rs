use std::{fs, path::PathBuf};

pub fn get_api_key() -> String {
    let data = fs::read_to_string(config_path()).expect("cyndra config file to exist");
    let toml: toml::Value = toml::from_str(&data).expect("to parse cyndra config file");

    toml["api_key"].to_string()
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("system to have a config path")
        .join("cyndra")
        .join("config.toml")
}

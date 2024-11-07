use std::{
    fs::{read_to_string, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use cargo_generate::{GenerateArgs, TemplatePath, Vcs};
use indoc::indoc;
use cyndra_common::project::ProjectName;
use toml_edit::{value, Document};

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Template {
    ActixWeb,
    Axum,
    Poise,
    Poem,
    Rocket,
    Salvo,
    Serenity,
    Tide,
    Thruster,
    Tower,
    Warp,
    None,
}

impl Template {
    /// Returns a framework-specific struct that implements the trait `CyndraInit`
    /// for writing framework-specific dependencies to `Cargo.toml` and generating
    /// boilerplate code in `src/main.rs`.
    pub fn init_config(&self) -> Box<dyn CyndraInit> {
        use Template::*;
        match self {
            ActixWeb => Box::new(CyndraInitActixWeb),
            Axum => Box::new(CyndraInitAxum),
            Rocket => Box::new(CyndraInitRocket),
            Tide => Box::new(CyndraInitTide),
            Tower => Box::new(CyndraInitTower),
            Poem => Box::new(CyndraInitPoem),
            Salvo => Box::new(CyndraInitSalvo),
            Serenity => Box::new(CyndraInitSerenity),
            Poise => Box::new(CyndraInitPoise),
            Warp => Box::new(CyndraInitWarp),
            Thruster => Box::new(CyndraInitThruster),
            None => Box::new(CyndraInitNoOp),
        }
    }
}

pub trait CyndraInit {
    fn get_repo_url(&self) -> &'static str;
    fn get_sub_path(&self) -> Option<&'static str>;
}

pub struct CyndraInitActixWeb;

impl CyndraInit for CyndraInitActixWeb {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("actix-web/hello-world")
    }
}

pub struct CyndraInitAxum;

impl CyndraInit for CyndraInitAxum {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("axum/hello-world")
    }
}

pub struct CyndraInitRocket;

impl CyndraInit for CyndraInitRocket {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("rocket/hello-world")
    }
}

pub struct CyndraInitTide;

impl CyndraInit for CyndraInitTide {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("tide/hello-world")
    }
}

pub struct CyndraInitPoem;

impl CyndraInit for CyndraInitPoem {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("poem/hello-world")
    }
}

pub struct CyndraInitSalvo;

impl CyndraInit for CyndraInitSalvo {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("salvo/hello-world")
    }
}

pub struct CyndraInitSerenity;

impl CyndraInit for CyndraInitSerenity {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("serenity/hello-world")
    }
}

pub struct CyndraInitPoise;

impl CyndraInit for CyndraInitPoise {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("poise/hello-world")
    }
}

pub struct CyndraInitTower;

impl CyndraInit for CyndraInitTower {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("tower/hello-world")
    }
}

pub struct CyndraInitWarp;

impl CyndraInit for CyndraInitWarp {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("warp/hello-world")
    }
}

pub struct CyndraInitThruster;

impl CyndraInit for CyndraInitThruster {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("thruster/hello-world")
    }
}

pub struct CyndraInitNoOp;
impl CyndraInit for CyndraInitNoOp {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/cyndra-hq/cyndra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("custom/none")
    }
}

pub fn cargo_generate(path: PathBuf, name: &ProjectName, framework: Template) -> Result<()> {
    let config = framework.init_config();

    println!(r#"    Creating project "{name}" in {path:?}"#);
    let generate_args = GenerateArgs {
        init: true,
        template_path: TemplatePath {
            git: Some(config.get_repo_url().to_string()),
            auto_path: config.get_sub_path().map(str::to_string),
            ..Default::default()
        },
        name: Some(name.to_string()), // appears to do nothing...
        destination: Some(path.clone()),
        vcs: Some(Vcs::Git),
        ..Default::default()
    };
    cargo_generate::generate(generate_args)
        .with_context(|| "Failed to initialize with cargo generate.")?;

    set_crate_name(&path, name.as_str()).with_context(|| "Failed to set crate name.")?;
    remove_cyndra_toml(&path);
    create_gitignore_file(&path).with_context(|| "Failed to create .gitignore file.")?;

    Ok(())
}

// since I can't get cargo-generate to do this for me...
fn set_crate_name(path: &Path, name: &str) -> Result<()> {
    // read the Cargo.toml file
    let mut path = path.to_path_buf();
    path.push("Cargo.toml");

    let toml_str = read_to_string(&path)?;
    let mut doc = toml_str.parse::<Document>()?;

    // change the name
    doc["package"]["name"] = value(name);

    // write the Cargo.toml file back out
    std::fs::write(&path, doc.to_string())?;

    Ok(())
}

/*
Currently Cyndra.toml only has a project name override.
This project name will already be in use, so the file is useless.

If we start putting more things in Cyndra.toml we may wish to re-evaluate.
*/
fn remove_cyndra_toml(path: &Path) {
    let mut path = path.to_path_buf();
    path.push("Cyndra.toml");

    // this file only exists for some of the examples, it's fine if we don't find it
    _ = std::fs::remove_file(path);
}

fn create_gitignore_file(path: &Path) -> Result<()> {
    let mut path = path.to_path_buf();
    path.push(".gitignore");

    let mut file = match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {
                    // if the example already has a .gitignore file, just use that
                    return Ok(());
                }
                _ => {
                    return Err(anyhow!(e));
                }
            }
        }
    };

    file.write_all(indoc! {b"
        /target
        Secrets*.toml
    "})?;

    Ok(())
}

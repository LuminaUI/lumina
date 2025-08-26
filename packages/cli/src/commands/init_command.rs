use crate::{
    Asset, NPM, inc_step,
    util::step::{CLIP, LOOKING_GLASS, PAPER, SPARKLE, Step, TRUCK, step},
};
use indicatif::{MultiProgress, style::TemplateError};
use log::error;
use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize, de::Deserializer};
use std::{
    env::current_dir,
    fs,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Could not find Config Json file")]
    ConfigJson,
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    TemplateError(#[from] TemplateError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Failed to install dependency {0}")]
    InstallError(String),
}

#[derive(Serialize, Debug)]
struct Config {
    base_dir: String,
    components_dir: String,
    lib_dir: String,
    registry: String,
    templates: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigRaw {
    components_dir: String,
    lib_dir: String,
    base_dir: String,
    registry: String,
    templates: String,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = ConfigRaw::deserialize(deserializer)?;

        Ok(Self {
            components_dir: raw.components_dir.replace("{baseDir}", &raw.base_dir),
            lib_dir: raw.lib_dir.replace("{baseDir}", &raw.base_dir),
            base_dir: raw.base_dir,
            registry: raw.registry,
            templates: raw.templates,
        })
    }
}

static PACKAGES: [&str; 1] = ["@rbxts/react"];

pub fn init_command(mp: &MultiProgress) -> Result<(), InitError> {
    let config_file = Asset::get("lumina.config.json").ok_or(InitError::ConfigJson)?;
    let config = serde_json::from_slice::<Config>(config_file.data.as_ref())?;

    let mut init_pb = Step::new(mp, 4, 4)?;

    step!(
        init_pb,
        CLIP,
        "Generating default files...",
        create_default_files(&config, &config_file)?
    );
    inc_step!(
        init_pb,
        LOOKING_GLASS,
        "Checking for dependencies...",
        check_for_required_deps(&mut init_pb)?
    );
    inc_step!(init_pb, SPARKLE, "Finished initializing lumina!");

    Ok(())
}

fn create_default_files(config: &Config, config_file: &EmbeddedFile) -> Result<(), InitError> {
    let project_dir = current_dir()?;

    fs::create_dir_all(&config.base_dir)?;
    fs::create_dir_all(&config.components_dir)?;
    fs::create_dir_all(&config.lib_dir)?;

    fs::write(project_dir.join("lumina.config.json"), &config_file.data)?;

    Ok(())
}

fn check_for_required_deps(pb: &mut Step) -> Result<(), InitError> {
    let mut installed = true;

    pb.inc();

    for pkg in PACKAGES {
        let exit_status = Command::new(NPM)
            .args(["ls", pkg, "--depth=0"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !exit_status.success() {
            step!(
                pb,
                TRUCK,
                "Installing required dependencies",
                install_dependencies(pkg, pb)?
            );
            installed = false;
        }
    }

    if installed {
        step!(pb, PAPER, "Dependencies already installed...");
    }

    Ok(())
}

fn install_dependencies(package: &str, pb: &Step) -> Result<(), InitError> {
    let exit_status = Command::new(NPM).args(["i", package]).status()?;

    if !exit_status.success() {
        pb.abandon();

        return Err(InitError::InstallError(package.to_string()));
    }

    Ok(())
}

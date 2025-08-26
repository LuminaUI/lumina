use crate::util::step::step;
use crate::{Asset, NPM};
use console::{Emoji, style};
use indicatif::style::TemplateError;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::error;
use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize, de::Deserializer};
use std::env::current_dir;
use std::fs;
use std::process::{Command, Stdio};
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
    base_dir: String,
    components_dir: String,
    lib_dir: String,
    registry: String,
    templates: String,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = ConfigRaw::deserialize(deserializer)?;
        let base = raw.base_dir;
        let components = raw.components_dir.replace("{baseDir}", &base);
        let lib = raw.lib_dir.replace("{baseDir}", &base);
        Ok(Config {
            base_dir: base,
            components_dir: components,
            lib_dir: lib,
            registry: raw.registry,
            templates: raw.templates,
        })
    }
}

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

static PACKAGES: [&str; 1] = ["@rbxts/react"];

pub fn init_command(mp: &MultiProgress) -> Result<(), InitError> {
    let config_file = Asset::get("lumina.config.json").ok_or_else(|| InitError::ConfigJson)?;
    let config = serde_json::from_slice::<Config>(config_file.data.as_ref())?;

    let init_pb = mp.add(ProgressBar::new(4));

    init_pb.set_style(
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")?
            .progress_chars("=>-"),
    );

    step(
        &init_pb,
        format!(
            "{} {}Generating default files...",
            style("[1/4]").bold().dim(),
            CLIP
        ),
        || create_default_files(&config, &config_file),
    )?;

    step(
        &init_pb,
        format!(
            "{} {}Checking for dependencies...",
            style("[2/4]").bold().dim(),
            LOOKING_GLASS
        ),
        || check_for_required_deps(&init_pb),
    )?;

    step(
        &init_pb,
        format!(
            "{} {}Finished initializing lumina!",
            style("[4/4]").bold().dim(),
            SPARKLE
        ),
        || (),
    );

    Ok(())
}

fn create_default_files(config: &Config, config_file: &EmbeddedFile) -> Result<(), InitError> {
    let project_dir = current_dir()?;

    fs::create_dir_all(&config.base_dir)?;
    fs::create_dir_all(&config.components_dir)?;
    fs::create_dir_all(&config.lib_dir)?;

    fs::write(&project_dir.join("lumina.config.json"), &config_file.data)?;

    Ok(())
}

fn check_for_required_deps(pb: &ProgressBar) -> Result<(), InitError> {
    let mut installed = true;
    for pkg in PACKAGES {
        let exit_status = Command::new(NPM)
            .args(&["ls", pkg, "--depth=0"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !exit_status.success() {
            step(
                pb,
                format!(
                    "{} {}Installing required dependencies",
                    style("[3/4]").bold().dim(),
                    TRUCK
                ),
                || install_dependencies(pkg, pb),
            )?;

            installed = false;
        }
    }

    if installed {
        step(
            pb,
            format!(
                "{} {}Dependencies already installed...",
                style("[3/4]").bold().dim(),
                PAPER
            ),
            || (),
        );
    }

    Ok(())
}

fn install_dependencies(package: &str, pb: &ProgressBar) -> Result<(), InitError> {
    let exit_status = Command::new(NPM).args(&["i", package]).status()?;

    if !exit_status.success() {
        pb.abandon();
        return Err(InitError::InstallError(package.to_string()));
    }

    Ok(())
}

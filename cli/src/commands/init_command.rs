use crate::{
    NPM,
    config::{Config, ConfigError},
    inc_step,
    preflights::init::{PreflightInitErrors, preflight_init},
    util::step::{LOOKING_GLASS, PAPER, SPARKLE, Step, TRUCK, step},
};
use console::style;
use dialoguer::Confirm;
use indicatif::{MultiProgress, style::TemplateError};
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fs::File,
    io::BufWriter,
    path::PathBuf,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    TemplateError(#[from] TemplateError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Failed to install dependency {0}")]
    InstallError(String),

    #[error("Failed to grab style `{0}`")]
    StyleNotFound(String),

    #[error(transparent)]
    PreflightError(#[from] PreflightInitErrors),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}

static PACKAGES: [&str; 1] = ["@rbxts/react"];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitSchema {
    pub cwd: PathBuf,
    pub yes: bool,
    pub force: bool,
    pub skip_preflight: bool,
}

pub fn init_command(mp: &MultiProgress, options: InitSchema) -> Result<(), InitError> {
    let mut init_pb = Step::new(mp, 5, 5)?;

    if !options.skip_preflight {
        step!(init_pb, TRUCK, "Starting preflight checks.");
        init_pb.pause();
        preflight_init(options.clone())?;

        if !&options.yes {
            let confirmation = Confirm::new()
                .with_prompt(format!(
                    "Write configuration to {}. Proceed?",
                    style("components.json").bold().cyan()
                ))
                .interact()
                .unwrap();

            if !confirmation {
                std::process::exit(0);
            }
        }

        init_pb.unpause();
    }

    step!(
        init_pb,
        PAPER,
        "Writing components.json",
        generate_components_json(&options)?
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

fn generate_components_json(options: &InitSchema) -> Result<(), InitError> {
    // TODO: Merge backup config if it exists and force is not being used
    let _backup_path = current_dir()?.join("components.json.bak");

    let file = File::create(options.cwd.join("components.json"))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &Config::default())?;

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

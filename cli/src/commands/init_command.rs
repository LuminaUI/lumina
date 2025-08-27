use crate::{
    LIBS, NPM, STYLES,
    config::Config,
    inc_step,
    util::step::{CLIP, LOOKING_GLASS, PAPER, SPARKLE, Step, TRUCK, step},
};
use include_dir::Dir;
use indicatif::{MultiProgress, style::TemplateError};
use log::error;
use std::{
    env::current_dir,
    fs::{self},
    path::Path,
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
}

static PACKAGES: [&str; 1] = ["@rbxts/react"];

pub fn init_command(mp: &MultiProgress) -> Result<(), InitError> {
    let mut init_pb = Step::new(mp, 4, 4)?;

    let config = crate::config::config();

    step!(
        init_pb,
        CLIP,
        "Generating default files...",
        create_default_files(&config)?
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

fn create_default_files(config: &Config) -> Result<(), InitError> {
    let project_dir = current_dir()?;

    // TODO: This will later be replaced from a select menu, for now there is only 1 theme so we will default to it.
    let default = STYLES
        .get_file("Default.rsml")
        .ok_or(InitError::StyleNotFound("theme.rsml".to_string()))?;

    fs::create_dir_all(&config.lib_dir)?;

    materialize_dir(&LIBS, &config.lib_dir)?;

    fs::create_dir_all(&config.styles_dir)?;
    fs::create_dir_all(&config.base_dir)?;
    fs::create_dir_all(&config.components_dir)?;

    fs::write(
        project_dir.join("lumina.config.json"),
        serde_json::to_string(&config)?,
    )?;

    fs::write(config.styles_dir.join("Default.rsml"), default.contents())?;

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

// TODO: Move this function somewhere else
fn materialize_dir(src: &Dir<'_>, dst: &Path) -> Result<(), InitError> {
    fs::create_dir_all(&dst)?;

    for entry in src.entries() {
        if let Some(file) = entry.as_file() {
            fs::write(&dst.join(file.path().file_name().unwrap()), file.contents())?;
        } else if let Some(dir) = entry.as_dir() {
            materialize_dir(dir, &dst.join(dir.path().file_name().unwrap()))?;
        }
    }

    Ok(())
}

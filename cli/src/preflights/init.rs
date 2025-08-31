use crate::commands::init_command::InitSchema;
use crate::util::get_project_info::get_project_info;
use crate::util::spinner::Spinner;
use console::{StyledObject, style};
use log::error;
use std::fmt::Debug;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreflightInitErrors {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("CWD could not be resolved.")]
    MissingCWD,
    #[error("package.json file was not found")]
    PackageJsonNotFound,
    #[error(
        "No import alias found in your tsconfig.json file, do so by adding {0} to your paths in tsconfig.json"
    )]
    NoImportAliasFound(String),
    #[error(
        "A {0} file already exists at {1}\nTo start over, remove the {2} file and run {3} again."
    )]
    ComponentsJsonExists(
        StyledObject<&'static str>,
        StyledObject<String>,
        StyledObject<&'static str>,
        StyledObject<&'static str>,
    ),
}

pub fn preflight_init(options: InitSchema) -> Result<(), PreflightInitErrors> {
    if !fs::exists(&options.cwd)? {
        return Err(PreflightInitErrors::MissingCWD);
    }

    if !fs::exists(&options.cwd.join("package.json"))? {
        return Err(PreflightInitErrors::PackageJsonNotFound);
    }

    let project_spinner = Spinner::spinner("Preflight checks.");

    if fs::exists(&options.cwd.join("components.json"))? {
        project_spinner.abandon();
        let cwd_string = options.cwd.to_string_lossy().to_string();
        let err = Err(PreflightInitErrors::ComponentsJsonExists(
            style("components.json").bold().cyan(),
            style(cwd_string).bold().dim(),
            style("components.json").bold().cyan(),
            style("init").bold(),
        ));
        return err;
    }

    project_spinner.finish();

    let project_info = get_project_info(&options.cwd);

    if project_info.is_none() {
        error!(
            "Roblox-ts is currently not set up, please run {} to do so",
            style("npm init roblox-ts game").bold().cyan()
        );
        std::process::exit(1);
    }

    let ts_config_spinner = Spinner::spinner("Validating import alias.");

    if project_info.unwrap().alias_prefix.is_none() {
        ts_config_spinner.abandon_with_message("Import Alias Missing");
        return Err(PreflightInitErrors::NoImportAliasFound(
            style("\"@/*\": [\"./src/shared/*\"]").bold().cyan().to_string(),
        ));
    }

    ts_config_spinner.finish();

    Ok(())
}

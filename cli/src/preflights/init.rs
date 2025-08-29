use crate::errors::Errors;
use crate::schemas::InitSchema;
use crate::util::get_project_info::get_project_info;
use crate::util::spinner::Spinner;
use console::style;
use log::error;
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreflightInitErrors {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn preflight_init(options: InitSchema) -> Result<HashMap<Errors, bool>, PreflightInitErrors> {
    let mut errors = HashMap::<Errors, bool>::new();

    if !fs::exists(&options.cwd)? || !fs::exists(&options.cwd.join("package.json"))? {
        errors.insert(Errors::MissingDirOrEmptyProject, true);
        return Ok(errors);
    }

    let project_spinner = Spinner::spinner("Preflight checks.");

    if fs::exists(&options.cwd.join("components.json"))? {
        project_spinner.abandon();
        error!(
            "A {} file already exists at {:#?}\nTo start over, remove the {} file and run {} again.",
            style("components.json").bold().cyan(),
            style(options.cwd).bold().dim(),
            style("components.json").bold().cyan(),
            style("init").bold()
        );
        std::process::exit(1);
    }

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
        errors.insert(Errors::ImportAliasMissing, true);
    }

    if errors.len() > 0 {
        if errors.get(&Errors::ImportAliasMissing).is_some() {
            error!(
                "No import alias found in your tsconfig.json file, do so by adding {} to your paths in tsconfig.json",
                style("\"@/*\": [\"./*\"]").bold().cyan()
            );
        }
    }

    Ok(errors)
}

use crate::commands::add_command::AddSchema;
use console::StyledObject;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreflightAdd {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("CWD could not be resolved.")]
    MissingCWD,
    #[error("package.json file was not found")]
    PackageJsonNotFound,
    #[error("components.json file is missing, please run init to start using lumina")]
    ComponentsJsonMissing,
}

pub fn preflight_add(options: &AddSchema) -> Result<(), PreflightAdd> {
    if !fs::exists(&options.cwd)? {
        return Err(PreflightAdd::MissingCWD);
    }

    if !fs::exists(&options.cwd.join("package.json"))? {
        return Err(PreflightAdd::PackageJsonNotFound);
    }

    if !fs::exists(options.cwd.join("components.json"))? {
        return Err(PreflightAdd::ComponentsJsonMissing);
    }

    Ok(())
}

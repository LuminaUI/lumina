use crate::commands::build_command::BuildSchema;
use console::style;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, path};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolvedPaths {
    pub cwd: PathBuf,
    pub registry: PathBuf,
    pub output: PathBuf,
}

#[derive(Error, Debug)]
pub enum PreflightError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Error does not exist")]
    ErrorDoesNotExist,
}

pub fn preflight_build(
    options: &BuildSchema,
) -> Result<(HashMap<String, bool>, ResolvedPaths), PreflightError> {
    let mut errors = HashMap::<String, bool>::new();

    let resolved_paths = ResolvedPaths {
        cwd: options.cwd.clone(),
        registry: path::absolute(&options.registry)?,
        output: path::absolute(&options.output)?,
    };

    if !fs::exists(&resolved_paths.registry)? {
        errors.insert(String::from("registry"), true);
    }

    fs::create_dir_all(&resolved_paths.output)?;

    if !errors.is_empty() {
        if *errors.get("registry").ok_or(PreflightError::ErrorDoesNotExist)? {
            error!(
                "The path {} does not exist",
                style(resolved_paths.registry.display()).bold().cyan()
            );
            std::process::exit(1);
        }
    }

    Ok((errors, resolved_paths))
}

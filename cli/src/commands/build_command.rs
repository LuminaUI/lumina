use crate::preflights::build::{PreflightError, preflight_build};
use crate::schemas::registry::Registry;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error(transparent)]
    PreflightError(#[from] PreflightError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildSchema {
    pub cwd: PathBuf,
    pub registry: PathBuf,
    pub output: PathBuf,
}

pub fn build_command(options: BuildSchema) -> Result<(), BuildError> {
    let (_, resolved_paths) = preflight_build(&options)?;

    let content = fs::read_to_string(&resolved_paths.registry)?;

    let mut result = serde_json::from_str::<Registry>(&content)?;

    for item in &mut result.items {
        item.schema = Some(String::from(
            "https://lumina-ui.com/schema/registry-item.json",
        ));

        for file in &mut item.files {
            let content = fs::read_to_string(&resolved_paths.cwd.join(&file.path))?;
            file.content = Some(content);
        }

        fs::write(
            &resolved_paths.output.join(format!("{}.json", item.name)),
            &serde_json::to_string_pretty(&item)?,
        )?;
    }

    fs::copy(
        &resolved_paths.registry,
        &resolved_paths.output.join("registry.json"),
    )?;

    info!("Build successful....");

    Ok(())
}

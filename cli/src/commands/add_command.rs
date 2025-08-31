use crate::config;
use crate::preflights::add::{PreflightAdd, preflight_add};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddError {
    #[error("Passed in components were empty")]
    ComponentsEmpty,
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    PreflightAdd(#[from] PreflightAdd),
    #[error(transparent)]
    RegistryError(#[from] RegistryError),
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryItem {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: RegistryType,
    pub description: String,
    pub title: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub registry_dependencies: Vec<String>,
    pub files: Vec<RegistryItemFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryItemFile {
    pub path: String,
    pub content: String,
    #[serde(rename = "type")]
    pub item_type: RegistryType,
    pub target: String,
    pub extends: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistryType {
    Block,
    Component,
    UI,
    Style,
}

impl Display for RegistryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryType::Block => write!(f, "registry:block"),
            RegistryType::Component => write!(f, "registry:component"),
            RegistryType::UI => write!(f, "registry:ui"),
            RegistryType::Style => write!(f, "registry:style"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSchema {
    pub cwd: PathBuf,
    pub components: Vec<String>,
}

pub async fn add_command(options: AddSchema) -> Result<(), AddError> {
    if options.components.is_empty() {
        return Err(AddError::ComponentsEmpty);
    }

    preflight_add(&options)?;

    let config = config::Config::get_config()?;

    add_components(&options.components, config, &options).await?;

    Ok(())
}

async fn add_components(
    components: &Vec<String>,
    _config: config::Config,
    _options: &AddSchema,
) -> Result<(), AddError> {
    for component in components {
        let registry_item = resolve_registry_item(component).await?;

        // We need to check if the component has any registry dependents, if so we need to add them as well

        if !registry_item.registry_dependencies.is_empty() {
            for _dep in registry_item.registry_dependencies {
                let _registry_dep = resolve_registry_item(component).await?;

                // We will then check if it already exists, if so skip otherwise we will add it
            }
        }

        // Next we need to loop over the files in the registry item and add them accordingly

        if !registry_item.files.is_empty() {
            for _file in registry_item.files {
                // We also need to check the type of the file, if it's a component we add it into whatever they specified for their components path, if it's a ui we add it to the ui path
            }
        }
    }
    // Check registry for component
    // Check if file already exists and whether to overwrite it or not, get confirmation
    // If all checks pass write to file at directory specified.
    Ok(())
}

// TODO: Move these functions to their own designated modules
async fn resolve_registry_item(component: &String) -> Result<RegistryItem, RegistryError> {
    // fetch the registry for the given component
    // TODO: We need to handle the errors better to be able to better report to the user, as right now it will just be a generic error from reqwest.
    let result = reqwest::get(format!("http://localhost:3000/r/{component}.json"))
        .await?
        .json::<RegistryItem>()
        .await?;

    Ok(result)
}

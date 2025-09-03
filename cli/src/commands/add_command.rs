use crate::util::get_project_info::get_project_info;
use crate::{
    config,
    preflights::add::{PreflightAdd, preflight_add},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
};
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
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryItem {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: RegistryType,
    pub description: String,
    pub title: String,
    pub author: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Vec<RegistryItemFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryItemFile {
    pub path: String,
    pub content: String,
    #[serde(rename = "type")]
    pub item_type: RegistryType,
    pub target: Option<String>,
    pub extends: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistryType {
    #[serde(rename = "registry:block")]
    Block,
    #[serde(rename = "registry:component")]
    Component,
    #[serde(rename = "registry:ui")]
    UI,
    #[serde(rename = "registry:style")]
    Style,
}

impl Display for RegistryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "registry:block"),
            Self::Component => write!(f, "registry:component"),
            Self::UI => write!(f, "registry:ui"),
            Self::Style => write!(f, "registry:style"),
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

    add_components(&options.components, &config, &options).await?;

    Ok(())
}

async fn add_components(
    components: &Vec<String>,
    config: &config::Config,
    options: &AddSchema,
) -> Result<(), AddError> {
    for component in components {
        let registry_item = resolve_registry_item(component).await?;

        // TODO: We need to check if the component has any registry dependents, if so we need to add them as well

        // Next we need to loop over the files in the registry item and add them accordingly

        let aliases = &config.aliases;

        if !registry_item.files.is_empty() {
            for file in registry_item.files {
                // We also need to check the type of the file, if it's a component we add it into whatever they specified for their components path, if it's a ui we add it to the ui path

                let project_info =
                    get_project_info(&options.cwd).expect("Couldn't get project info");
                let project_paths = match file.item_type {
                    RegistryType::Component => project_info
                        .aliases_paths
                        .get(aliases.components.as_ref().unwrap().as_str()),
                    RegistryType::UI => {
                        project_info.aliases_paths.get(aliases.ui.as_ref().unwrap().as_str())
                    }
                    _ => None,
                };

                let paths = project_paths
                    .map(|paths| {
                        paths
                            .iter()
                            .map(|p| {
                                let s = p.to_string_lossy();
                                PathBuf::from(s.trim_end_matches("/*"))
                            })
                            .collect::<Vec<PathBuf>>()
                    })
                    .unwrap_or_default();

                for path in paths {
                    let component_path = path.join(&registry_item.name);
                    fs::create_dir_all(&component_path)?;
                    fs::write(
                        &component_path.join(format!("{}.tsx", &registry_item.name)),
                        &file.content,
                    )?;
                }
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
    let result = reqwest::get(format!("https://lumina-ui.com/r/{component}.json"))
        .await?
        .json::<RegistryItem>()
        .await?;

    Ok(result)
}

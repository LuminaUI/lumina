use crate::config::Config;
use crate::schemas::registry_item::RegistryItem;
use crate::schemas::registry_item_file::RegistryItemFile;
use crate::schemas::registry_type::RegistryType;
use crate::{
    config,
    preflights::add::{PreflightAdd, preflight_add},
};
use serde::{Deserialize, Serialize};
use std::{fs, path, path::PathBuf};
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
    #[error("Content of component is empty")]
    ContentEmpty,
    #[error("Component contains an invalid registry type")]
    InvalidRegistryType,
    #[error(
        "Could not resolve the path for components, check your tsconfig paths and ensure the directories exist"
    )]
    CouldNotResolveTargetPath,
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
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

    let config = Config::get_config()?;

    add_components(&options.components, &config, &options).await?;

    Ok(())
}

async fn add_components(
    components: &Vec<String>,
    config: &Config,
    options: &AddSchema,
) -> Result<(), AddError> {
    for component in components {
        let registry_item = resolve_registry_item(component).await?;

        // TODO: We need to check if the component has any registry dependents, if so we need to add them as well

        // Next we need to loop over the files in the registry item and add them accordingly

        if !registry_item.files.is_empty() {
            for file in registry_item.files {
                let target_dir = resolve_file_target_path(&file, &config);

                if target_dir.is_none() {
                    return Err(AddError::CouldNotResolveTargetPath);
                }

                let target_dir = target_dir.unwrap();

                let component_path = target_dir.join(&registry_item.name);
                fs::create_dir_all(&component_path)?;

                let (name, ext) = match file.item_type {
                    RegistryType::Component => ("index", "tsx"),
                    RegistryType::UI => ("index", "tsx"),
                    RegistryType::Style => (registry_item.name.as_str(), "rsml"),
                    _ => ("index", "tsx"),
                };

                fs::write(
                    &component_path.join(format!("{}.{}", name, ext)),
                    &file.content.clone().ok_or(AddError::ContentEmpty)?,
                )?;
            }
        }

        // if !registry_item.files.is_empty() {
        //     for file in registry_item.files {
        //         // We also need to check the type of the file, if it's a component we add it into whatever they specified for their components path, if it's a ui we add it to the ui path
        //
        //         let project_info =
        //             get_project_info(&options.cwd).expect("Couldn't get project info");
        //
        //         let project_paths = match file.item_type {
        //             RegistryType::Component => project_info
        //                 .aliases_paths
        //                 .get(aliases.components.as_ref().unwrap().as_str()),
        //             RegistryType::UI => {
        //                 project_info.aliases_paths.get(aliases.ui.as_ref().unwrap().as_str())
        //             }
        //             _ => None,
        //         };
        //
        //         let paths = project_paths
        //             .map(|paths| {
        //                 paths
        //                     .iter()
        //                     .map(|p| {
        //                         let s = p.to_string_lossy();
        //                         PathBuf::from(s.trim_end_matches("/*"))
        //                     })
        //                     .collect::<Vec<PathBuf>>()
        //             })
        //             .unwrap_or_default();
        //
        //         for path in paths {
        //             let component_path = path.join(&registry_item.name);
        //             fs::create_dir_all(&component_path)?;
        //             fs::write(
        //                 &component_path.join(format!("{}.tsx", &registry_item.name)),
        //                 &file.content.clone().ok_or(AddError::ContentEmpty)?,
        //             )?;
        //
        //             if file.item_type == RegistryType::Style {
        //                 fs::write(
        //                     &component_path.join(format!("{}.rsml", &registry_item.name)),
        //                     &file.content.clone().ok_or(AddError::ContentEmpty)?,
        //                 )?;
        //             }
        //         }
        //     }
        // }
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

fn resolve_file_target_path(file: &RegistryItemFile, config: &Config) -> Option<PathBuf> {
    return match file.item_type {
        RegistryType::Component => config.resolved_paths.components.clone(),
        RegistryType::Block => config.resolved_paths.components.clone(),
        RegistryType::UI => config.resolved_paths.ui.clone(),
        _ => config.resolved_paths.components.clone(),
    };
}

use std::{fs, path::PathBuf, str::FromStr};

use indicatif::MultiProgress;
use log::info;
use thiserror::Error;

use crate::{COMPONENTS, config::config};

#[derive(Debug, Error)]
pub enum AddError {
    #[error("Could not find component `{0}` in allowed components.")]
    ComponentNotFound(String),
    #[error("Component missing main file.")]
    ComponentFileMissing,
    #[error("Component RSML file is missing")]
    RSMLFileMissing,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// TODO: Can't be asked to make it use a progress bar right now, will eventually use one to display the progress of adding the components
pub fn add_command(components: &[String], _mp: &MultiProgress) -> Result<(), AddError> {
    for component in components {
        add_component(component)?;
    }
    Ok(())
}

fn add_component(component: &str) -> Result<(), AddError> {
    let component_path = &PathBuf::from_str(component).unwrap();

    let component_dir = COMPONENTS
        .get_dir(component_path)
        .ok_or(AddError::ComponentNotFound(component.to_string()))?;

    let component_file =
        component_dir.get_file("index.tsx").ok_or(AddError::ComponentFileMissing)?;

    let mut c = component.chars();

    let rsml_file_name = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };

    let rsml_file = component_dir
        .get_file(format!("{}.rsml", rsml_file_name))
        .ok_or(AddError::RSMLFileMissing)?;

    let user_components_path = config().components_dir.join(component_path);
    fs::create_dir_all(&user_components_path)?;

    let user_component_file_path = &user_components_path.join(component_file.path());

    fs::write(user_component_file_path, component_file.contents())?;

    let user_rsml_file_path = &user_components_path.join(rsml_file.path());
    fs::write(user_rsml_file_path, rsml_file.contents())?;

    info!("Added {component} at the path {:?}", user_components_path);

    Ok(())
}

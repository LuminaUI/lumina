use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("There is no current directory")]
    NoCurrentDir,
    #[error("There is no components.json at `{0}`")]
    NoComponentsJson(String),
    #[error("components.json file is empty")]
    EmptyComponentsJson,
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aliases {
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utils: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hooks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lib: Option<String>,
}

// Plans for new themes later on.
#[derive(Serialize, Deserialize, Debug)]
pub enum Themes {
    Default,
}

impl Themes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Themes::Default => "default",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: Themes,
    pub aliases: Aliases,
}

impl Config {
    pub fn new() -> Config {
        Self {
            theme: Themes::Default,
            aliases: Aliases {
                components: Some(String::from("@components/")),
                ui: Some(String::from("@components/ui")),
                utils: None,
                lib: None,
                hooks: None,
            },
        }
    }

    pub fn get_config() -> Result<Config, ConfigError> {
        let current_directory = current_dir().map_err(|_| ConfigError::NoCurrentDir)?;
        let config_path = current_directory.join("components.json");

        let mut f = File::open(config_path).map_err(|_| {
            ConfigError::NoComponentsJson(current_directory.to_string_lossy().to_string())
        })?;
        let mut contents = String::new();
        f.read_to_string(&mut contents).map_err(|_| ConfigError::EmptyComponentsJson)?;

        Ok(serde_json::from_str::<Config>(&contents)?)
    }
}

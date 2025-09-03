use serde::{Deserialize, Serialize};
use std::{env::current_dir, fs::File, io::BufReader};
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
    pub components: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utils: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<String>,
}

impl Default for Aliases {
    #[inline(always)]
    fn default() -> Self {
        Self {
            components: Some(String::from("@/components")),
            ui: Some(String::from("@/components/ui")),
            utils: None,
            hooks: None,
            lib: None,
        }
    }
}

// Plans for new themes later on.
#[derive(Serialize, Deserialize, Debug)]
pub enum Themes {
    Default,
}

impl Themes {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
        }
    }
}

impl Default for Themes {
    #[inline(always)]
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub theme: Themes,
    pub aliases: Aliases,
}

impl Config {
    pub fn get_config() -> Result<Config, ConfigError> {
        let current_directory = current_dir().map_err(|_| ConfigError::NoCurrentDir)?;
        let config_path = current_directory.join("components.json");

        Ok(serde_json::from_reader::<_, Config>(BufReader::new(
            File::open(config_path).map_err(|_| {
                ConfigError::NoComponentsJson(current_directory.to_string_lossy().to_string())
            })?,
        ))?)
    }
}

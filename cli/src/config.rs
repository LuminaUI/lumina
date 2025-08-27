use std::{path::PathBuf, sync::OnceLock};

use serde::{Deserialize, Serialize, de::Deserializer};
use thiserror::Error;

use crate::ASSETS;

#[derive(Serialize, Debug)]
pub struct Config {
    pub base_dir: PathBuf,
    pub components_dir: PathBuf,
    pub styles_dir: PathBuf,
    pub lib_dir: PathBuf,
    pub registry: String,
    pub templates: String,
    pub themes: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigRaw {
    components_dir: String,
    lib_dir: String,
    styles_dir: String,
    base_dir: String,
    registry: String,
    templates: String,
    themes: String,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = ConfigRaw::deserialize(deserializer)?;

        Ok(Self {
            components_dir: raw.components_dir.replace("{baseDir}", &raw.base_dir).into(),
            lib_dir: raw.lib_dir.replace("{baseDir}", &raw.base_dir).into(),
            styles_dir: raw.styles_dir.replace("{baseDir}", &raw.base_dir).into(),
            base_dir: raw.base_dir.into(),
            registry: raw.registry,
            templates: raw.templates,
            themes: raw.themes,
        })
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not find the requested file: `{0}`")]
    AssetError(String),
    #[error("transparent")]
    SerdeError(#[from] serde_json::Error),
    #[error("Config already initialized")]
    AlreadyInitialized,
    #[error("Config not initialized")]
    NotInitialized,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() -> Result<(), ConfigError> {
    let config_file = ASSETS
        .get_file("lumina.config.json")
        .ok_or(ConfigError::AssetError("lumina.config.json".to_string()))?;

    let config = serde_json::from_slice::<Config>(config_file.contents())?;

    CONFIG.set(config).map_err(|_| ConfigError::AlreadyInitialized)?;

    Ok(())
}

pub fn config() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}

#[allow(dead_code)]
pub fn try_config() -> Result<&'static Config, ConfigError> {
    CONFIG.get().ok_or(ConfigError::NotInitialized)
}

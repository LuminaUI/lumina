use crate::util::get_project_info::get_project_info;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::PathBuf;
use std::{env::current_dir, fs, fs::File, io::BufReader};
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
    #[error(transparent)]
    Io(#[from] std::io::Error),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedPaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utils: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<PathBuf>,
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

impl Default for ResolvedPaths {
    #[inline(always)]
    fn default() -> Self {
        Self {
            cwd: Some(current_dir().unwrap()),
            components: None,
            utils: None,
            ui: None,
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
    #[serde(skip)]
    pub resolved_paths: ResolvedPaths,
}

impl Config {
    pub fn get_config() -> Result<Config, ConfigError> {
        let current_directory = current_dir().map_err(|_| ConfigError::NoCurrentDir)?;
        let config_path = current_directory.join("components.json");
        let mut config = serde_json::from_reader::<_, Config>(BufReader::new(
            File::open(config_path).map_err(|_| {
                ConfigError::NoComponentsJson(current_directory.to_string_lossy().to_string())
            })?,
        ))?;

        config.resolved_paths = resolve_config_paths(&config)?;

        Ok(config)
    }
}

pub fn write_config(new_config: &Config) -> Result<(), ConfigError> {
    let current_directory = current_dir().map_err(|_| ConfigError::NoCurrentDir)?;
    let config_path = current_directory.join("components.json");

    let data = serde_json::to_string_pretty(&new_config)?;

    fs::write(&config_path, data.as_bytes())?;

    Ok(())
}

pub fn resolve_config_paths(config: &Config) -> Result<ResolvedPaths, ConfigError> {
    let current_dir = current_dir().map_err(|_| ConfigError::NoCurrentDir)?;
    let project_info = get_project_info(&current_dir).unwrap();

    let ui = project_info.aliases_paths.get("@/ui").map(|path| {
        let mut dir = current_dir.join(path);
        dir.pop();
        dir
    });
    let components = project_info.aliases_paths.get("@/components").map(|path| {
        let mut dir = current_dir.join(path);
        dir.pop();
        dir
    });
    let utils = project_info.aliases_paths.get("@/utils").map(|path| {
        let mut dir = current_dir.join(path);
        dir.pop();
        dir
    });
    let hooks = project_info.aliases_paths.get("@/hooks").map(|path| {
        let mut dir = current_dir.join(path);
        dir.pop();
        dir
    });
    let lib = project_info.aliases_paths.get("@/lib").map(|path| {
        let mut dir = current_dir.join(path);
        dir.pop();
        dir
    });

    Ok(ResolvedPaths {
        cwd: Some(current_dir.clone()),
        components,
        ui,
        utils,
        hooks,
        lib,
    })
}

use log::error;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum TsAliasError {
    #[error("Failed to read tsconfig at {0}: {1}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("Failed to parse tsconfig JSON at {0}: {1}")]
    Json(PathBuf, #[source] serde_json::Error),
}

#[derive(Debug, Deserialize)]
struct TsConfig {
    #[serde(rename = "compilerOptions")]
    compiler_options: Option<CompilerOptions>,
}

#[derive(Debug, Deserialize)]
struct CompilerOptions {
    #[serde(default)]
    paths: Option<HashMap<String, Vec<String>>>,
}

pub struct ProjectInfo {
    pub is_src_dir: bool,
    pub is_tsx: bool,
    pub alias_prefix: Option<String>,
}

pub fn get_project_info(cwd: &PathBuf) -> Option<ProjectInfo> {
    let is_src_dir = fs::exists(cwd.join("src")).ok()?;
    let is_tsx = is_typescript_project(cwd)?;

    Some(ProjectInfo {
        is_tsx,
        is_src_dir,
        alias_prefix: get_ts_config_alias_prefix(cwd).ok()?,
    })
}

pub fn is_typescript_project(cwd: &PathBuf) -> Option<bool> {
    fs::exists(cwd.join("tsconfig.json")).ok()
}

pub fn get_ts_config_alias_prefix(cwd: &PathBuf) -> Result<Option<String>, TsAliasError> {
    let tsconfig_path = cwd.join("tsconfig.json");
    let raw = fs::read(&tsconfig_path).map_err(|e| TsAliasError::Io(tsconfig_path.clone(), e))?;
    let cfg: TsConfig =
        serde_json::from_slice(&raw).map_err(|e| TsAliasError::Json(tsconfig_path.clone(), e))?;

    let paths_map_opt: Option<HashMap<String, Vec<String>>> =
        cfg.compiler_options.and_then(|co| co.paths);

    let Some(paths_map) = paths_map_opt else {
        return Ok(None);
    };

    if paths_map.is_empty() {
        return Ok(None);
    };

    const CANDIDATES: &[&str] = &["./*", "./src/*"];

    for (alias, targets) in &paths_map {
        let hits_candidate = targets.iter().any(|t| CANDIDATES.iter().any(|c| t == c));
        if hits_candidate {
            return Ok(Some(trim_alias_suffix(alias.as_str())));
        }
    }

    if let Some(first_key) = paths_map.keys().next() {
        return Ok(Some(trim_alias_suffix(first_key.as_str())));
    }

    Ok(None)
}

fn trim_alias_suffix(alias: &str) -> String {
    alias.strip_suffix("/*").unwrap_or(alias).to_string()
}

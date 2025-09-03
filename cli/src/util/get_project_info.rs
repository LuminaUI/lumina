use log::error;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum TsAliasError {
    #[error("Failed to read tsconfig at {0}: {1}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("Failed to parse tsconfig JSON at {0}: {1}")]
    Json(PathBuf, #[source] serde_json::Error),
    #[error("Failed to strip comments from the tsconfig file at {0}: {1}")]
    StripComments(PathBuf, #[source] std::io::Error),
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
    pub aliases_paths: HashMap<String, Vec<PathBuf>>,
}

pub fn get_project_info(cwd: &Path) -> Option<ProjectInfo> {
    let is_src_dir = fs::exists(cwd.join("src")).ok()?;
    let is_tsx = is_typescript_project(cwd)?;

    Some(ProjectInfo {
        is_tsx,
        is_src_dir,
        aliases_paths: get_aliases_paths(cwd).ok()?,
    })
}

pub fn is_typescript_project(cwd: &Path) -> Option<bool> {
    fs::exists(cwd.join("tsconfig.json")).ok()
}

pub fn get_aliases_paths(cwd: &Path) -> Result<HashMap<String, Vec<PathBuf>>, TsAliasError> {
    let tsconfig_path = cwd.join("tsconfig.json");
    let mut raw = fs::read_to_string(&tsconfig_path)
        .map_err(|e| TsAliasError::Io(tsconfig_path.clone(), e))?;

    json_strip_comments::strip(raw.as_mut_str())
        .map_err(|e| TsAliasError::StripComments(tsconfig_path.clone(), e))?;
    let cfg: TsConfig = serde_json::from_str(raw.as_str())
        .map_err(|e| TsAliasError::Json(tsconfig_path.clone(), e))?;

    let allowed_prefixes = ["@/component", "@/ui"];

    let aliases = cfg
        .compiler_options
        .and_then(|co| co.paths)
        .map(|paths| {
            paths
                .into_iter()
                .filter(|(alias, _)| {
                    allowed_prefixes.iter().any(|prefix| alias.starts_with(prefix))
                })
                .map(|(k, v)| {
                    let new_value = v.into_iter().map(PathBuf::from).collect::<Vec<_>>();
                    let new_key = trim_suffix(k.as_str());
                    (new_key, new_value)
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(aliases)
}

pub fn trim_suffix(alias: &str) -> String {
    alias.strip_suffix("/*").unwrap_or(alias).to_string()
}

pub fn write_paths(cwd: &Path) -> Result<(), TsAliasError> {
    let tsconfig_path = cwd.join("tsconfig.json");
    let mut raw = fs::read_to_string(&tsconfig_path)
        .map_err(|e| TsAliasError::Io(tsconfig_path.clone(), e))?;
    json_strip_comments::strip(raw.as_mut_str())
        .map_err(|e| TsAliasError::StripComments(tsconfig_path.clone(), e))?;
    let mut root: serde_json::Value = serde_json::from_str(raw.as_str())
        .map_err(|e| TsAliasError::Json(tsconfig_path.clone(), e))?;

    if let Some(co) = root.get_mut("compilerOptions").and_then(Value::as_object_mut) {
        co.insert(
            "paths".to_string(),
            json!({
                "@/ui/*": ["src/shared/components/ui/*"],
                "@/components/*": ["src/shared/components/*"]
            }),
        );

        co.insert("baseUrl".to_string(), json!("."));
    }

    fs::write(
        &tsconfig_path,
        serde_json::to_string_pretty(&root)
            .map_err(|e| TsAliasError::Json(tsconfig_path.clone(), e))?,
    )
    .map_err(|e| TsAliasError::Io(tsconfig_path.clone(), e))?;

    Ok(())
}

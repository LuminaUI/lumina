use crate::schemas::registry_item_file::RegistryItemFile;
use crate::schemas::registry_type::RegistryType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryItem {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: RegistryType,
    pub description: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_dependencies: Option<Vec<String>>,
    pub files: Vec<RegistryItemFile>,
}

use crate::schemas::registry_item::RegistryItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub name: String,
    pub homepage: String,
    pub items: Vec<RegistryItem>,
}

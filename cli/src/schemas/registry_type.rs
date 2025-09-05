use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RegistryType {
    #[serde(rename = "registry:block")]
    Block,
    #[serde(rename = "registry:component")]
    Component,
    #[serde(rename = "registry:ui")]
    UI,
    #[serde(rename = "registry:style")]
    Style,
}

impl Display for RegistryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "registry:block"),
            Self::Component => write!(f, "registry:component"),
            Self::UI => write!(f, "registry:ui"),
            Self::Style => write!(f, "registry:style"),
        }
    }
}

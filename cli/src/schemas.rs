use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use zod_rs::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitSchema {
    pub cwd: PathBuf,
    pub yes: bool,
    pub force: bool,
    pub skip_preflight: bool,
}

pub fn init_schema() -> impl Schema<Value> {
    object()
        .field("cwd", string())
        .field("yes", boolean())
        .field("force", boolean())
        .field("skip_preflight", boolean())
}

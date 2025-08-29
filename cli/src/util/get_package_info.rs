use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    // Basic metadata
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub private: Option<bool>,
    #[serde(rename = "type")]
    pub module_type: Option<ModuleType>, // "module" | "commonjs"
    pub main: Option<String>,
    pub module: Option<String>,
    pub types: Option<String>,
    pub typings: Option<String>,
    pub browser: Option<Value>, // string | object
    pub exports: Option<Value>, // string | object | array
    pub files: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<Value>, // string | object
    pub homepage: Option<String>,
    #[serde(default)]
    pub package_manager: Option<String>,

    // People/links
    pub author: Option<Person>,
    pub contributors: Option<Vec<Person>>,
    pub repository: Option<Repository>,
    pub bugs: Option<Bugs>,
    pub funding: Option<Funding>, // string | object | array

    // Scripts & config
    pub scripts: Option<BTreeMap<String, String>>,
    pub config: Option<BTreeMap<String, Value>>,

    // Dependencies
    pub dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "bundleDependencies")]
    pub bundle_dependencies: Option<Value>, // bool | string[] (alias of bundledDependencies)
    #[serde(rename = "bundledDependencies")]
    pub bundled_dependencies: Option<Value>, // bool | string[]

    // Engines / OS / CPU
    pub engines: Option<BTreeMap<String, String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,

    // Bin can be a string or map
    pub bin: Option<Bin>,

    // Workspaces can be array or object ({ packages, nohoist, ... })
    pub workspaces: Option<Value>,

    // Publish config, eslint/ts/jest/etc configs are often inlined as objects
    #[serde(rename = "publishConfig")]
    pub publish_config: Option<BTreeMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleType {
    Module,
    Commonjs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Single(String),
    Map(BTreeMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Repository {
    Url(String), // e.g. "github:user/repo"
    Info {
        r#type: Option<String>,    // "git"
        url: Option<String>,       // "git+https://github.com/user/repo.git"
        directory: Option<String>, // "packages/foo"
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Person {
    /// "Jane Doe <jane@example.com> (https://example.com)"
    StringForm(String),
    /// { "name": "...", "email": "...", "url": "..." }
    ObjectForm {
        name: Option<String>,
        email: Option<String>,
        url: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bugs {
    Url(String),
    Object {
        url: Option<String>,
        email: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Funding {
    StringForm(String),
    ObjectForm {
        r#type: Option<String>,
        url: Option<String>,
    },
    ArrayForm(Vec<FundingSingle>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingSingle {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub url: Option<String>,
}

pub fn get_package_info(cwd: &PathBuf) -> Option<PackageJson> {
    let package_json_path = cwd.join("package.json");

    File::open(&package_json_path).ok().and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Reading package.json");

        Some(serde_json::from_str::<PackageJson>(&contents).unwrap())
    })
}

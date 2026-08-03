use std::{fs, path::Path};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::constants::paths::PACKAGE_JSON;


#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
    pub dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<IndexMap<String, String>>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
    pub engines: Option<IndexMap<String, String>>,
    #[serde(rename = "scripts-info")]
    pub scripts_info: Option<IndexMap<String, String>>,
}

/// Load package.json from the current directory. (从当前目录加载 package.json)
pub fn load() -> Result<PackageJson> {
    load_from(Path::new("."))
}

/// Load package.json from a specific project directory. (从指定目录加载 package.json)
pub fn load_from(project_path: &Path) -> Result<PackageJson> {
    let package_path = project_path.join(PACKAGE_JSON);
    let content = fs::read_to_string(&package_path)
        .with_context(|| format!("failed to read {}", package_path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse {}", package_path.display()))
}
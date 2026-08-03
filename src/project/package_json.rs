use std::{
    collections::HashMap,
    fs,
    path::Path,
};
use serde::Deserialize;
use crate::constants::paths::PACKAGE_JSON;


#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

/// Load package.json from project directory. (从项目目录加载 package.json)
pub fn load(project_path: &Path) -> Option<PackageJson> {
    let package_path = project_path.join(PACKAGE_JSON);
    let content = fs::read_to_string(package_path)
        .ok()?;

    serde_json::from_str(&content)
        .ok()
}
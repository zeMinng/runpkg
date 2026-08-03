use std::collections::HashMap;
use super::package_json::PackageJson;


#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub scripts: HashMap<String, String>,
}

impl From<PackageJson> for ProjectInfo {
    fn from(pkg: PackageJson) -> Self {
        Self {
            name: pkg
                .name
                .unwrap_or_else(|| "unknown".into()),
            
            version: pkg
                .version
                .unwrap_or_else(|| "0.0.0".into()),
            
            scripts: pkg
                .scripts
                .unwrap_or_default(),
        }
    }
}
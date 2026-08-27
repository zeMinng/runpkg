use indexmap::IndexMap;
use super::package_json::PackageJson;


#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub scripts: IndexMap<String, String>,
    pub dependencies: Vec<DepEntry>,
    pub dev_dependencies: Vec<DepEntry>,
    pub peer_dependencies: Vec<DepEntry>,
    pub package_manager: Option<String>,
    pub engines: IndexMap<String, String>,
}

#[derive(Debug)]
pub struct DepEntry {
    pub name: String,
    pub version: String,
}

impl From<PackageJson> for ProjectInfo {
    fn from(pkg: PackageJson) -> Self {
        Self {
            name: pkg.name.unwrap_or_else(|| "unknown".into()),
            version: pkg.version.unwrap_or_else(|| "0.0.0".into()),
            scripts: pkg.scripts.unwrap_or_default(),
            dependencies: map_to_entries(pkg.dependencies),
            dev_dependencies: map_to_entries(pkg.dev_dependencies),
            peer_dependencies: map_to_entries(pkg.peer_dependencies),
            package_manager: pkg.package_manager,
            engines: pkg.engines.unwrap_or_default(),
        }
    }
}

fn map_to_entries(map: Option<IndexMap<String, String>>) -> Vec<DepEntry> {
    map.unwrap_or_default()
        .into_iter()
        .map(|(name, version)| DepEntry { name, version })
        .collect()
}

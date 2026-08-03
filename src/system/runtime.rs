use super::node;

#[derive(Debug)]
pub struct RuntimeInfo {
    pub node_version: Option<String>,
}

pub fn collect_runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        node_version: node::version(),
        // package_manager: None,
        // git_branch: None,
    }
}
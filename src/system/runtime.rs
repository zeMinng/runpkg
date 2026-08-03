use super::{node, pm};

#[derive(Debug)]
pub struct RuntimeInfo {
    pub node_version: Option<String>,
    pub available_package_managers: Vec<String>,
}

pub async fn collect_runtime_info() -> RuntimeInfo {
    let (node_version, available_package_managers) =
        tokio::join!(node::version(), pm::detect_available());

    RuntimeInfo {
        node_version,
        available_package_managers,
    }
}
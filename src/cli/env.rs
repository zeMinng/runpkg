use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct TargetPkgInfo {
    pub name: Option<String>,
    pub version: Option<String>,
}

/// 获取目标 Node.js 项目的 package.json 信息
pub fn get_target_project_info() -> Option<TargetPkgInfo> {
    let cwd = env::current_dir().ok()?;
    let pkg_path = cwd.join("package.json");

    if !pkg_path.exists() {
        return None;
    }

    let content = fs::read_to_string(pkg_path).ok()?;
    serde_json::from_str(&content).ok()
}

/// 获取系统本地 Node.js 版本
pub fn get_local_node_version() -> Option<String> {
    let output = Command::new("node").arg("-v").output();
    match output {
        Ok(out) if out.status.success() => {
            Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
        }
        _ => None,
    }
}
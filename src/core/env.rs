use std::process::Command;

pub struct ProjectEnv {
    pub project_name: String,
    pub version: String,
    pub node_version: String,
    pub pm_type: String,
    pub git_branch: String,
}

impl ProjectEnv {
    pub fn detect() -> Self {
        // 1. 自动侦测当前 Node 版本
        let node_version = Command::new("node")
            .arg("-v")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "v? (未安装)".to_string());

        // 2. 自动侦测 Git 分支
        let git_branch = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        // 3. 识别包管理器
        let pm_type = if std::path::Path::new("pnpm-lock.yaml").exists() {
            "pnpm"
        } else if std::path::Path::new("yarn.lock").exists() {
            "yarn"
        } else if std::path::Path::new("bun.lockb").exists() {
            "bun"
        } else {
            "npm"
        }
        .to_string();

        // 4. 解析 package.json
        let (project_name, version) = match std::fs::read_to_string("package.json") {
            Ok(content) => {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let name = json["name"]
                        .as_str()
                        .unwrap_or("unnamed-project")
                        .to_string();
                    let ver = json["version"].as_str().unwrap_or("0.0.0").to_string();
                    (name, ver)
                } else {
                    ("invalid-pkg-json".to_string(), "0.0.0".to_string())
                }
            }
            Err(_) => ("no-package-json".to_string(), "0.0.0".to_string()),
        };

        Self {
            project_name,
            version,
            node_version,
            pm_type,
            git_branch,
        }
    }
}
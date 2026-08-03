use std::process::Command;

/// Get the version of the Node.js installed on the system. (获取系统本地 Node.js 版本)
pub fn version() -> Option<String> {
    Command::new("node")
        .arg("-v")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .to_string()
        })
}
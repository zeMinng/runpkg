use tokio::process::Command;
use std::time::Duration;

/// Get the version of the Node.js installed on the system. (获取系统本地 Node.js 版本)
pub async fn version() -> Option<String> {
    let output = tokio::time::timeout(
        Duration::from_secs(5),
        Command::new("node").arg("-v").output(),
    )
    .await
    .ok()?
    .ok()?;

    output
        .status
        .success()
        .then(|| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string()
        })
}
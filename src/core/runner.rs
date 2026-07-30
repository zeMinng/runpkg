use anyhow::{Context, Result};
use std::process::Command;

pub fn exec_script(script: &str, args: &[String]) -> Result<()> {
    let pm = if std::path::Path::new("pnpm-lock.yaml").exists() {
        "pnpm"
    } else if std::path::Path::new("yarn.lock").exists() {
        "yarn"
    } else if std::path::Path::new("bun.lockb").exists() {
        "bun"
    } else {
        "npm"
    };

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", pm, "run", script]);
        c
    } else {
        let mut c = Command::new(pm);
        c.args(["run", script]);
        c
    };

    cmd.args(args);

    let mut child = cmd
        .spawn()
        .with_context(|| format!("无法启动包管理器: {}", pm))?;

    let status = child.wait()?;
    if !status.success() {
        eprintln!(
            "❌ 脚本执行失败，退出代码: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
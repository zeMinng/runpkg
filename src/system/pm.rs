use std::time::Duration;
use tokio::process::Command;

/// Well-known package managers in priority order.
const PACKAGE_MANAGERS: &[&str] = &["pnpm", "bun", "yarn", "npm"];

/// Build a Command to run a package manager. On Windows, `.cmd`/`.bat` files
/// need `cmd /c` to execute; we also append `.cmd` to avoid picking up shell scripts
/// that happen to share the same name (e.g. `pnpm` the bash wrapper vs `pnpm.cmd`).
fn build_command(pm: &str) -> Command {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        let exe = format!("{}.cmd", pm);
        cmd.args(["/c", &exe, "--version"]);
        cmd
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = Command::new(pm);
        cmd.arg("--version");
        cmd
    }
}

/// Detect which package managers are available on the system PATH.
pub async fn detect_available() -> Vec<String> {
    let mut available = Vec::new();

    for pm in PACKAGE_MANAGERS {
        let ok = tokio::time::timeout(
            Duration::from_secs(2),
            build_command(pm).output(),
        )
        .await
        .map(|r| r.map(|o| o.status.success()).unwrap_or(false))
        .unwrap_or(false);

        if ok {
            available.push(pm.to_string());
        }
    }

    available
}

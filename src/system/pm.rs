use std::path::Path;
use std::time::Duration;
use tokio::process::Command;

use crate::constants::paths::LOCK_FILES;
use crate::project::info::ProjectInfo;
use crate::system::runtime::RuntimeInfo;

/// Well-known package managers in priority order.
/// The boolean marks whether the command is a `.cmd` shim on Windows
/// (npm / pnpm / yarn ship `.cmd` wrappers; bun is a native `bun.exe`).
const PACKAGE_MANAGERS: &[(&str, bool)] = &[
    ("pnpm", true),
    ("bun", false),
    ("yarn", true),
    ("npm", true),
];

/// Build a Command to run a package manager's `--version`.
/// On Windows, `.cmd`/`.bat` files must be executed through `cmd /c`; native
/// executables (bun) are spawned directly.
fn build_command(pm: &str, is_cmd_shim: bool) -> Command {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        let target = if is_cmd_shim {
            format!("{}.cmd", pm)
        } else {
            pm.to_string()
        };
        cmd.args(["/c", &target, "--version"]);
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

    for (pm, is_cmd_shim) in PACKAGE_MANAGERS {
        let ok = tokio::time::timeout(
            Duration::from_secs(2),
            build_command(pm, *is_cmd_shim).output(),
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

/// Resolve which package manager to run scripts with.
/// Priority:
/// 1. `project.packageManager` field (explicit declaration) — name before `@`.
/// 2. Lock file signal (`pnpm-lock.yaml` → pnpm, etc.).
/// 3. First package manager available on PATH (pnpm > bun > yarn > npm).
/// 4. `npm` fallback.
pub fn preferred(
    project: Option<&ProjectInfo>,
    runtime: Option<&RuntimeInfo>,
    project_path: &Path,
) -> String {
    if let Some(name) = project
        .and_then(|p| p.package_manager.as_deref())
        .and_then(|s| s.split('@').next())
    {
        return name.to_string();
    }

    if let Some(pm) = lock_file_pm(project_path) {
        return pm.to_string();
    }

    if let Some(pm) = runtime.and_then(|r| r.available_package_managers.first()) {
        return pm.clone();
    }

    "npm".to_string()
}

/// Infer the package manager from the presence of a lock file.
fn lock_file_pm(project_path: &Path) -> Option<&'static str> {
    LOCK_FILES
        .iter()
        .find(|(file, _)| project_path.join(file).exists())
        .map(|(_, pm)| *pm)
}

/// Build the command that runs `script` via `pm`.
/// Returns `(program, args)`; on Windows, `.cmd` shims run through `cmd /c`.
pub fn build_run_command(pm: &str, script: &str) -> (String, Vec<String>) {
    #[cfg(target_os = "windows")]
    {
        if is_cmd_shim(pm) {
            return (
                "cmd".to_string(),
                vec![
                    "/c".to_string(),
                    format!("{pm}.cmd"),
                    "run".to_string(),
                    script.to_string(),
                ],
            );
        }
    }

    (pm.to_string(), vec!["run".to_string(), script.to_string()])
}

/// Whether `pm` ships a `.cmd`/`.bat` wrapper on Windows (npm/pnpm/yarn do; bun does not).
fn is_cmd_shim(pm: &str) -> bool {
    PACKAGE_MANAGERS
        .iter()
        .find(|(name, _)| *name == pm)
        .map(|(_, shim)| *shim)
        .unwrap_or(true)
}

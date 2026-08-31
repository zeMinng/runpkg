//! Lightweight Git introspection for the header. (供 header 使用的轻量 Git 状态检测)

use std::path::Path;
use std::process::Command;

/// Snapshot of the current Git working-tree state.
#[derive(Debug, Default, Clone)]
pub struct GitInfo {
    /// Current branch name; `None` when not inside a Git repository.
    pub branch: Option<String>,
    /// Tracked files changed (staged or unstaged) — rendered as `*`.
    pub modified: bool,
    /// New / untracked files — rendered as `+`.
    pub added: bool,
    /// Deletions — rendered as `-`.
    pub deleted: bool,
}

/// Collect branch name and change indicators in a single `git status` call.
///
/// Uses `git -C <path> status --porcelain=v1 -b`, whose first line (`## ...`)
/// carries the branch, and the rest are `XY <file>` entries. Any failure
/// (not a repo, `git` missing) degrades to `GitInfo::default()`.
pub fn status(repo_path: &Path) -> GitInfo {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("status")
        .arg("--porcelain=v1")
        .arg("-b")
        .output();

    let Ok(output) = output else {
        return GitInfo::default();
    };
    if !output.status.success() {
        return GitInfo::default();
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut info = GitInfo::default();

    for (idx, line) in text.lines().enumerate() {
        if idx == 0 {
            info.branch = parse_branch(line);
            continue;
        }

        let code = line.get(..2).unwrap_or("");
        match code {
            "??" => info.added = true,
            _ => {
                if code.contains(['M', 'A', 'R', 'C']) {
                    info.modified = true;
                }
                if code.contains('D') {
                    info.deleted = true;
                }
            }
        }
    }

    info
}

/// Extract the branch name from the leading `## ...` line.
fn parse_branch(line: &str) -> Option<String> {
    let rest = line.strip_prefix("## ")?;
    let name = rest
        .split(['.', '[', ' '])
        .next()
        .filter(|s| !s.is_empty())?;

    // `HEAD (no branch)` means a detached HEAD; normalize to a stable label.
    if name == "HEAD" {
        return Some("HEAD".to_string());
    }
    Some(name.to_string())
}

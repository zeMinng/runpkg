#[allow(dead_code)]
//! Package.json and lock files constants. (包含 package.json 和锁文件常量)

/// Package.json file name. (package.json 文件名)
pub const PACKAGE_JSON: &str = "package.json";
/// Lock files. (锁文件，供 doctor 子命令使用)
pub const LOCK_FILES: &[&str] = &[
    "pnpm-lock.yaml",
    "package-lock.json",
    "yarn.lock",
    "bun.lock",
];

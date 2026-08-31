//! Package.json and lock files constants. (包含 package.json 和锁文件常量)
#[allow(dead_code)]

/// Package.json file name. (package.json 文件名)
pub const PACKAGE_JSON: &str = "package.json";
/// Lock file → package manager signal, in priority order (pnpm > bun > yarn > npm).
pub const LOCK_FILES: &[(&str, &str)] = &[
    ("pnpm-lock.yaml", "pnpm"),
    ("bun.lock", "bun"),
    ("yarn.lock", "yarn"),
    ("package-lock.json", "npm"),
];

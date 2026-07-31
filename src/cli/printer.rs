use colored::Colorize;
use crate::constants::{ASCII_BANNER, SLOGAN};

/// 打印炫酷的起始欢迎语（比如在子命令执行前调用）
pub fn print_welcome_banner() {
    println!("{}", ASCII_BANNER.cyan().bold());
    println!("  {}\n", SLOGAN.yellow());
}

/// 如果你想完全自定义 `-v` 触发时的排版，也可以调用这个函数
pub fn print_custom_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{} {}", "📦 runpkg".bold().green(), format!("v{}", version).cyan());
    println!("  引擎: Rust (edition 2021)");
}
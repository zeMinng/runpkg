//! Application metadata. (包含应用名称、版本、描述、作者等基础信息)

/// Terminal startup banner. (终端启动 ASCII Banner / LOGO)
pub const BANNER: &str = r#"
 ██████╗ ██╗   ██╗███╗   ██╗██████╗ ██╗  ██╗ ██████╗
 ██╔══██╗██║   ██║████╗  ██║██╔══██╗██║ ██╔╝██╔════╝
 ██████╔╝██║   ██║██╔██╗ ██║██████╔╝█████╔╝ ██║  ███╗
 ██╔══██╗██║   ██║██║╚██╗██║██╔═══╝ ██╔═██╗ ██║   ██║
 ██║  ██║╚██████╔╝██║ ╚████║██║     ██║  ██╗╚██████╔╝
 ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝ ╚═════╝
"#;

/// Application name. (项目名)
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Application version from Cargo.toml. (应用版本)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application description from Cargo.toml. (应用描述)
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Application authors from Cargo.toml. (应用作者)
pub const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Application credits. (应用版权信息)
pub const APP_CREDITS: &str = "Developed with ❤️  by \x1b[1;35mzeMinng\x1b[0m";

/// print project name and app version. (打印项目的项目名和版本号)
pub fn print_app_info() { println!("{APP_NAME}, v{APP_VERSION}"); }

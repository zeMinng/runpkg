pub mod cli;
pub mod constants;
pub mod project;
pub mod system;

use crate::constants::app::{APP_NAME, APP_VERSION};
use cli::Commands;
use project::{info::ProjectInfo, package_json};
use system::runtime::collect_runtime_info;

/// Print name and version. 打印应用名称与版本
pub fn print_app_info() {
    println!("{} v{}", APP_NAME, APP_VERSION);
}

/// Perform core initialization and logic scheduling. 执行核心初始化与逻辑调度
pub async fn run_app(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    print_app_info();

    // Async collect runtime info (Node version + available PMs). 异步的收集运行时信息（Node 版本 + 可用的包管理器）
    let runtime = collect_runtime_info().await;
    println!("{:?}", runtime);

    // Load package.json from the specified project directory. 加载 package.json
    match package_json::load_from(&args.project_path) {
        Ok(package) => {
            let project: ProjectInfo = package.into();
            println!("{:#?}", project);
        }
        Err(e) => {
            eprintln!("Warning: {}", e);
        }
    }

    // Dispatch to subcommand. 调度子命令
    let action = match args.command {
        Some(Commands::Scripts) => "Scripts",
        Some(Commands::Deps) => "Manage deps",
        Some(Commands::Doctor) => "Project doctor",
        None => "Start runpkg TUI",
    };
    println!("{}", action);

    Ok(())
}



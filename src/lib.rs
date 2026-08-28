pub mod app;
pub mod cli;
pub mod constants;
pub mod project;
pub mod system;
pub mod tui;

use std::path::{Path, PathBuf};

use cli::Commands;
use project::{info::ProjectInfo, package_json};
use system::runtime::collect_runtime_info;


/// Unified entry point: dispatch to a subcommand or the TUI. (统一入口：分派到子命令或 TUI)
pub async fn run(args: cli::Args) -> anyhow::Result<()> {
    let project_path = args.resolve_project_path()?;
    match args.command {
        Some(command) => run_cli(project_path, command).await,
        None => run_tui(project_path).await,
    }
}

/// CLI subcommand path (`scripts` / `deps` / `doctor`).
async fn run_cli(project_path: PathBuf, command: Commands) -> anyhow::Result<()> {

    let runtime = collect_runtime_info().await;
    println!("{runtime:?}");

    match package_json::load_from(&project_path) {
        Ok(package) => {
            let project: ProjectInfo = package.into();
            println!("{project:#?}");
        }
        Err(e) => eprintln!("Warning: {e}"),
    }

    let action = match command {
        Commands::Scripts => "Scripts",
        Commands::Deps => "Manage deps",
        Commands::Doctor => "Project doctor",
    };
    println!("{action}");

    Ok(())
}

/// TUI path: gather runtime + project data, then run the event loop. (TUI 路径：收集运行时项目数据，然后运行事件循环)
async fn run_tui(project_path: PathBuf) -> anyhow::Result<()> {
    let (runtime, project) = tokio::join!(
        collect_runtime_info(),
        async { load_project(&project_path) }
    );

    let mut app = app::App::new(app::AppState::new(project, Some(runtime)), project_path);
    tui::run(&mut app)
}

fn load_project(project_path: &Path) -> Option<ProjectInfo> {
    package_json::load_from(project_path)
        .ok()
        .map(ProjectInfo::from)
}

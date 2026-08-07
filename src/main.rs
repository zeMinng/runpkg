mod cli;
mod constants;
mod system;
mod project;

use crate::constants::app::{
    APP_NAME,
    APP_VERSION,
};
use clap::Parser;
use cli::{Args, Commands};
use system::runtime::collect_runtime_info;
use project::{
    package_json,
    info::ProjectInfo,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Print banner + version
    println!("{} v{}", APP_NAME, APP_VERSION);

    // Collect runtime info (async — Node version + available PMs)
    let runtime = collect_runtime_info().await;
    println!("{:?}", runtime);

    // Load package.json from the specified project directory
    match package_json::load_from(&args.project_path) {
        Ok(package) => {
            let project: ProjectInfo = package.into();
            println!("{:#?}", project);
        }
        Err(e) => {
            eprintln!("Warning: {}", e);
        }
    }

    // Dispatch to subcommand
    let action = match args.command {
        Some(Commands::Scripts) => "Scripts",
        Some(Commands::Deps)    => "Manage deps",
        Some(Commands::Doctor)  => "Project doctor",
        None                    => "Start runpkg TUI",
    };
    println!("{}", action);

    println!("\nRunning... Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;

    Ok(())
}

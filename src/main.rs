mod cli;
mod constants;

use crate::constants::app::{
    APP_NAME,
    APP_VERSION,
    APP_DESCRIPTION,
    APP_AUTHORS,
};
use clap::Parser;
use cli::{
    Args,
    Commands,
    get_target_project_info,
    get_local_node_version,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{} v{} {} {}", APP_NAME, APP_VERSION, APP_AUTHORS, APP_DESCRIPTION);

    if let Some(node_ver) = get_local_node_version() {
        println!("Node 版本: {}", node_ver);
    }

    if let Some(pkg) = get_target_project_info() {
        println!("项目: {:?}", pkg.name);
    }

    match args.command {
        Some(Commands::Scripts) => {
            println!("Scripts");
        }

        Some(Commands::Deps) => {
            println!("Manage deps");
        }

        Some(Commands::Doctor) => {
            println!("Project doctor");
        }

        None => {
            println!("Start runpkg TUI");
        }
    }

    Ok(())
}
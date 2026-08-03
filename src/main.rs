mod cli;
mod constants;
mod system;
mod project;

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
};
use system::runtime::collect_runtime_info;
use std::path::Path;
use project::{
    package_json,
    info::ProjectInfo,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{} v{} {} {}", APP_NAME, APP_VERSION, APP_AUTHORS, APP_DESCRIPTION);

    let runtime = collect_runtime_info();
    println!("{:?}", runtime.node_version);

    let package = package_json::load(
        Path::new(".")
    );
    let Some(package) = package else {
        println!("package.json not found");
        return Ok(());
    };
    let project: ProjectInfo = package.into();
    println!("{:#?}", project);


    match runtime.node_version {
        Some(version) => {
            println!("Node版本: {}", version);
        }
        None => {
            println!("没有检测到 Node");
        }
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
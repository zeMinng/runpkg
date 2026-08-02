mod cli;
mod constants;

use crate::constants::app::{
    APP_NAME,
    APP_VERSION,
    APP_DESCRIPTION,
    APP_AUTHORS,
};
use clap::Parser;
use cli::{Args, Commands};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{} v{} {} {}", APP_NAME, APP_VERSION, APP_AUTHORS, APP_DESCRIPTION);

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
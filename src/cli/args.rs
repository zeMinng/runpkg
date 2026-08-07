use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::constants::app::{
    BANNER,
    APP_NAME,
    APP_VERSION,
    APP_DESCRIPTION,
    APP_AUTHORS,
    APP_CREDITS,
};

/// runpkg CLI arguments. (运行 runpkg 命令行参数)
#[derive(Parser, Debug)]
#[command(
    name = APP_NAME,
    bin_name = APP_NAME,
    author = APP_AUTHORS,
    version = APP_VERSION,
    about = APP_DESCRIPTION,
    before_help = BANNER,
    after_help = APP_CREDITS
)]
pub struct Args {
    /// Project directory path (default: current directory)
    #[arg(short = 'p', long = "path", default_value = ".")]
    pub project_path: PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run package scripts
    Scripts,

    /// Manage dependencies
    Deps,

    /// Check project status
    Doctor,
}
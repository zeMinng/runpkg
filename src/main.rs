use anyhow::Context;
use clap::Parser;
use runpkg::{cli::Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args).await.context("Failed to run runpkg application")
}

use clap::Parser;
use runpkg::{cli::Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run(Args::parse()).await
}

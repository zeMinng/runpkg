use clap::Parser;
use runpkg::{cli::Args, run_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Invoke the logic exposed by the lib. 调用lib暴露的逻辑
    run_app(args).await?;

    // Handling global signal interrupts. 处理全局信号中断
    println!("\nRunning... Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;

    Ok(())
}

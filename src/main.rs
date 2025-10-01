mod cli;
mod simulator;
mod logger;
mod accounts;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if let Err(e) = simulator::run(args).await {
        eprintln!("❌ Simulation failed: {}", e);
    }
}

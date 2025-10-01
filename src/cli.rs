use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "solfoundry")]
#[command(about = "Solana Foundry - Transaction Simulator", long_about = None)]
pub struct Cli {
    /// Program ID to simulate
    #[arg(short, long)]
    pub program: String,

    /// Path to transaction JSON file
    #[arg(short, long)]
    pub tx: String,

    /// Verbose logs
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

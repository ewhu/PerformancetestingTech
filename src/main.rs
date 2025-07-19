// src/main.rs
/*
 * Main executable for PerformancetestingTech
 */

use clap::Parser;
use performancetestingtech::{Result, run};

#[derive(Parser)]
#[command(version, about = PerformancetestingTech - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

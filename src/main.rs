// src/main.rs
/*
 * Main executable for ArtificialCanvasGenerator
 */

use clap::Parser;
use artificialcanvasgenerator::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialCanvasGenerator - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

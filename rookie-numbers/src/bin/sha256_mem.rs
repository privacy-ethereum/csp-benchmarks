//! Memory measurement binary for Rookie Numbers SHA256 prover.
//!
//! This binary is used by the harness to measure peak memory usage
//! during proof generation.

use clap::Parser;
use rookie_numbers::{prepare, prove};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter in bytes
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_mem(args.input_size);
}

fn sha256_mem(input_size: usize) {
    // Prepare the prover context
    let ctx = prepare(input_size);

    // Generate the proof (this is what we're measuring memory for)
    let _proof = prove(&ctx);
}

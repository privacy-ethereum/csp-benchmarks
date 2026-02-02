//! Memory measurement binary for Rookie Numbers SHA256 prover.
//!
//! This binary is used by the harness to measure peak memory usage
//! during proof generation.

use clap::Parser;
use rookie_numbers::{secure_pcs_config, MAX_PREPROCESSED_LOG_SIZE};
use sha256::{preprocess_sha256, prove_sha256};

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
    let config = secure_pcs_config();

    // Preprocess (this is part of what we measure)
    let preprocessed = preprocess_sha256(MAX_PREPROCESSED_LOG_SIZE, config);

    // Prepare the prover context
    let (message_bytes, _digest) = utils::generate_sha256_input(input_size);

    // Generate the proof (this is what we're measuring memory for)
    let _proof = prove_sha256(&message_bytes, config, &preprocessed);
}

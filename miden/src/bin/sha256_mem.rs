use clap::Parser;
use miden::{prepare_sha256, prove_sha256, verify_sha256};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let prepared = prepare_sha256(args.input_size);
    let proof = prove_sha256(&prepared);
    verify_sha256(&prepared, &proof);
}

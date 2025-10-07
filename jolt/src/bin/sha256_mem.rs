use clap::Parser;
use jolt::{prepare_sha256, prove_sha256};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let prepared = prepare_sha256(args.input_size);
    prove_sha256(&prepared);
}

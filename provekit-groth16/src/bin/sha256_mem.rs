use clap::Parser;
use provekit_groth16_bench::{prepare_sha256, prove};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = prepare_sha256(args.input_size);
    let _proof = prove(&prepared);
}

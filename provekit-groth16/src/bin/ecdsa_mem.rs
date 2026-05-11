use clap::Parser;
use provekit_groth16_bench::{prepare_ecdsa, prove};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = prepare_ecdsa(args.input_size);
    let _proof = prove(&prepared);
}

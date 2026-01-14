use circom::keccak::{prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    keccak_mem(args.input_size);
}

fn keccak_mem(input_size: usize) {
    let (witness_fn, input_str, zkey_path) = prepare(input_size);
    let _ = prove(witness_fn, input_str, zkey_path);
}

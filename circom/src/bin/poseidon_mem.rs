use circom::poseidon::{prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter (number of field elements)
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    poseidon_mem(args.input_size);
}

fn poseidon_mem(input_size: usize) {
    let (witness_fn, input_str, zkey_path) = prepare(input_size);
    let _ = prove(witness_fn, input_str, zkey_path);
}

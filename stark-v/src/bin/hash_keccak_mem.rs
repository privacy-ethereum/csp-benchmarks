//! Memory measurement binary for stark-v hash_keccak prover.

use clap::Parser;
use stark_v::{load_compiled, prepare_hash_keccak, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled("hash_keccak");
    let prepared = prepare_hash_keccak(args.input_size, &program);
    prove_bench(&prepared, &program);
}

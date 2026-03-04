//! Memory measurement binary for stark-v Keccak prover.

use clap::Parser;
use stark_v::{load_compiled, prepare_keccak, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the Keccak benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled("keccak");
    let prepared = prepare_keccak(args.input_size, &program);
    prove_bench(&prepared, &program);
}

//! Memory measurement binary for stark-v private_tx prover.

use clap::Parser;
use stark_v::{load_compiled, prepare_private_tx, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    /// Merkle depth for the private_tx benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled("private_tx");
    let prepared = prepare_private_tx(args.input_size, &program);
    prove_bench(&prepared, &program);
}

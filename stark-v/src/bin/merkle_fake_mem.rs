//! Memory measurement binary for stark-v merkle_fake prover.

use clap::Parser;
use stark_v::{load_compiled, prepare_merkle_fake, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled("merkle_fake");
    let prepared = prepare_merkle_fake(args.input_size, &program);
    prove_bench(&prepared, &program);
}

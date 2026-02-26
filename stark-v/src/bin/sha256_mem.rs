//! Memory measurement binary for stark-v SHA256 prover.

use clap::Parser;
use stark_v::{load_compiled, prepare_sha256, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled("sha256");
    let prepared = prepare_sha256(args.input_size, &program);
    prove_bench(&prepared, &program);
}

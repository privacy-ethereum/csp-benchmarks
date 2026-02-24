//! Memory measurement binary for stark-v SHA256 prover.

use clap::Parser;
use stark_v::{prepare_sha256, prove_sha256};
use stark_v_sdk::StarkVCompiler;
use utils::zkvm::helpers::load_compiled_program;
use utils::zkvm::SHA256_BENCH;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<StarkVCompiler>(SHA256_BENCH);
    let prepared = prepare_sha256(args.input_size, &program);
    prove_sha256(&prepared, &());
}

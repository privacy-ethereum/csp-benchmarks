//! Memory measurement binary for stark-v Keccak prover.

use clap::Parser;
use stark_v::{prepare_keccak, prove};
use stark_v_sdk::StarkVCompiler;
use utils::zkvm::helpers::load_compiled_program;
use utils::zkvm::KECCAK_BENCH;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the Keccak benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<StarkVCompiler>(KECCAK_BENCH);
    let prepared = prepare_keccak(args.input_size, &program);
    prove(&prepared, &());
}

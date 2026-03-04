use clap::Parser;
use jolt::JoltCompiler;
use jolt::{prepare_keccak, prove};
use utils::zkvm::KECCAK_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the Keccak benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<JoltCompiler>(KECCAK_BENCH);
    let prepared = prepare_keccak(args.input_size, &program);
    prove(&prepared, &());
}

use clap::Parser;
use ere_jolt::compiler::RustRv64imacCustomized;
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
    let program = load_compiled_program::<RustRv64imacCustomized>(KECCAK_BENCH);
    let prepared = prepare_keccak(args.input_size, &program);
    prove(&prepared, &());
}

use clap::Parser;
use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{prepare_blake3, prove_blake3};
use utils::zkvm::BLAKE3_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the BLAKE3 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv64imacCustomized>(BLAKE3_BENCH);
    let prepared = prepare_blake3(args.input_size, &program);
    prove_blake3(&prepared, &());
}

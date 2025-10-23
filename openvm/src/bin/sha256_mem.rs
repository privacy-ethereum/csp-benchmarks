use clap::Parser;
use ere_openvm::compiler::RustRv32imaCustomized;
use openvm::{prepare_sha256, prove_sha256};
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv32imaCustomized>(SHA256_BENCH);

    let prepared = prepare_sha256(args.input_size, &program);
    prove_sha256(&prepared, &());
}

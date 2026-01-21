use clap::Parser;
use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{prepare_keccak, prove_keccak};
use utils::zkvm::KECCAK_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv32imaCustomized>(KECCAK_BENCH);
    let prepared = prepare_keccak(args.input_size, &program);
    prove_keccak(&prepared, &());
}

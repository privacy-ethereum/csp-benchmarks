use clap::Parser;
use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{prepare_hash_keccak, prove_targeted};
use utils::zkvm::HASH_KECCAK_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv32imaCustomized>(HASH_KECCAK_BENCH);
    let prepared = prepare_hash_keccak(args.input_size, &program);
    prove_targeted(&prepared, &());
}

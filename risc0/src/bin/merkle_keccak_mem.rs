use clap::Parser;
use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{prepare_merkle_keccak, prove_targeted};
use utils::zkvm::MERKLE_KECCAK_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv32imaCustomized>(MERKLE_KECCAK_BENCH);
    let prepared = prepare_merkle_keccak(args.input_size, &program);
    prove_targeted(&prepared, &());
}

use clap::Parser;
use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{prepare_merkle_keccak, prove_sha256};
use utils::zkvm::MERKLE_KECCAK_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv64imacCustomized>(MERKLE_KECCAK_BENCH);
    let prepared = prepare_merkle_keccak(args.input_size, &program);
    prove_sha256(&prepared, &());
}

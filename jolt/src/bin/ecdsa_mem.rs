use clap::Parser;
use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{prepare_ecdsa, prove_ecdsa};
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size (unused for ECDSA)
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv64imacCustomized>(ECDSA_BENCH);
    let prepared = prepare_ecdsa(args.input_size, &program);
    prove_ecdsa(&prepared, &());
}

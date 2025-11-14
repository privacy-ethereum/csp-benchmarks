use clap::Parser;
use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{prepare_ecdsa, prove_ecdsa};
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Unused parameter for compatibility with benchmark harness
    #[arg(long = "input-size")]
    input_size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<RustRv32imaCustomized>(ECDSA_BENCH);

    let prepared = prepare_ecdsa(args.input_size.unwrap_or(1), &program);

    prove_ecdsa(&prepared, &());
}

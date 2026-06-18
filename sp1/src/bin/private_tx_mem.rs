use clap::Parser;
use ere_sp1::compiler::RustRv64imaCustomized;
use sp1::{prepare_private_tx, prove_private_tx};
use utils::zkvm::PRIVATE_TX_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Merkle depth for the private_tx benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<RustRv64imaCustomized>(PRIVATE_TX_BENCH);

    let prepared = prepare_private_tx(args.input_size, &program);
    prove_private_tx(&prepared, &());
}

use clap::Parser;
use ere_nexus::compiler::RustRv32i;
use nexus::{prepare_keccak, prove};
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
    let program = load_compiled_program::<RustRv32i>(KECCAK_BENCH);

    let prepared = prepare_keccak(args.input_size, &program);
    prove(&prepared, &());
}

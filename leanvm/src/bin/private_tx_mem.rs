use clap::Parser;
use leanvm::{compile_private_tx, prepare_private_tx, prove_private_tx};

#[derive(Parser)]
struct Args {
    /// Merkle depth for the private_tx benchmark.
    #[arg(long, env = "INPUT_SIZE")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let bytecode = compile_private_tx();
    let prepared = prepare_private_tx(args.input_size, &bytecode);
    let _proof = prove_private_tx(&prepared, &());
}

use clap::Parser;
use evm_sp1::{prepare_private_tx, prove_private_tx, setup_private_tx};

#[derive(Parser)]
struct Args {
    /// Merkle depth for the private_tx benchmark.
    #[arg(long, default_value_t = evm_sp1::PRIVATE_TX_DEPTH)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let state = setup_private_tx();
    let prepared = prepare_private_tx(args.input_size, &state);
    let _proof = prove_private_tx(&prepared, &&state);
}

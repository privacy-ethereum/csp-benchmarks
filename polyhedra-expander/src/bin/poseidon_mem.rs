use clap::Parser;
use gkr_engine::MPIConfig;
use sha256_expander_benchmark::poseidon::{prepare, prove};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    poseidon_mem(args.input_size);
}

fn poseidon_mem(input_size: usize) {
    let (circuit_bytes, witness_bytes) = prepare(input_size);
    let universe = MPIConfig::init().expect("Failed to initialize MPI");
    let world = universe.world();
    let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));
    let _proof = prove(&circuit_bytes, &witness_bytes, mpi_config);
}

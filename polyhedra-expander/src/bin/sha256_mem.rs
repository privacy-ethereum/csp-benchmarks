use clap::Parser;
use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::{prepare, prove};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_no_lookup_mem(args.input_size);
}

fn sha256_no_lookup_mem(input_size: usize) {
    let (circuit_file, witness_file) = prepare(input_size);
    let universe = MPIConfig::init().expect("Failed to initialize MPI");
    let world = universe.world();
    let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));
    let _proof = prove(&circuit_file, &witness_file, mpi_config);
}

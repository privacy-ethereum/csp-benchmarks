use bin::executor::dump_proof_and_claimed_v;
use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use utils::metadata::SHA2_INPUTS;

fn main() {
    //TODO refactor to measure RAM for varying input sizes
    let (circuit_file, witness_file) = prepare(SHA2_INPUTS[0]);

    let universe = MPIConfig::init().unwrap();
    let world = universe.world();
    let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));

    let (claimed_v, proof) = prove(&circuit_file, &witness_file, mpi_config.clone());

    let proof_bytes = dump_proof_and_claimed_v(&proof, &claimed_v).unwrap();

    println!("Proof size: {:.3} KB", proof_bytes.len() as f64 / 1024.0);
}

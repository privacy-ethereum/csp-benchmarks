use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;
use utils::harness::{BenchHarnessConfig, ProvingSystem};

utils::define_benchmark_harness!(
    sha256_expander_benches,
    {
        let universe = MPIConfig::init().expect("Failed to initialize MPI");
        let world = universe.world();
        (universe, world)
    },
    BenchHarnessConfig::sha256(ProvingSystem::Expander, None, Some("sha256_mem")),
    |size, _| prepare(size),
    |prepared_context, (universe, world)| {
        let (circuit_file, witness_file) = prepared_context;
        let (_claimed, proof) = prove(
            circuit_file,
            witness_file,
            MPIConfig::prover_new(Some(&universe), Some(&world)),
        );
        proof
    },
    |prepared_context, proof, (universe, world)| {
        let (circuit_file, witness_file) = prepared_context;
        let (claimed, _) = prove(
            circuit_file,
            witness_file,
            MPIConfig::prover_new(Some(&universe), Some(&world)),
        );
        verify(
            circuit_file,
            witness_file,
            proof,
            &claimed,
            MPIConfig::prover_new(Some(&universe), Some(&world)),
        );
    },
    |prepared_context, _shared| {
        let (circuit_file, _witness_file) = prepared_context;
        std::fs::metadata(circuit_file).unwrap().len() as usize
    },
    |proof, _shared| proof.bytes.len()
);

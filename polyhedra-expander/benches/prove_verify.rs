use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;
use utils::harness::BenchTarget;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Expander,
    None,
    "sha256_mem",
    {
        let universe = MPIConfig::init().expect("Failed to initialize MPI");
        let world = universe.world();
        (universe, world)
    },
    |size, _| prepare(size),
    |(circuit_file, witness_file), (universe, world)| {
        let (_claimed, proof) = prove(
            circuit_file,
            witness_file,
            MPIConfig::prover_new(Some(&universe), Some(&world)),
        );
        proof
    },
    |(circuit_file, witness_file), proof, (universe, world)| {
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
    |(circuit_file, _witness_file), _shared| {
        std::fs::metadata(circuit_file).unwrap().len() as usize
    },
    |proof, _shared| proof.bytes.len()
);

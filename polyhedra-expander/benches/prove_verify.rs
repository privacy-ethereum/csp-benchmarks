use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Expander,
    None,
    "sha256_mem_expander",
    {
        let universe = MPIConfig::init().expect("Failed to initialize MPI");
        let world = universe.world();
        (universe, world)
    },
    |size, _| prepare(size),
    |(circuit_bytes, witness_bytes), (universe, world)| {
        let (_, proof) = prove(
            circuit_bytes,
            witness_bytes,
            MPIConfig::prover_new(Some(universe), Some(world)),
        );
        proof
    },
    |(circuit_bytes, witness_bytes), proof, (universe, world)| {
        let (claimed, _) = prove(
            circuit_bytes,
            witness_bytes,
            MPIConfig::prover_new(Some(universe), Some(world)),
        );
        verify(
            circuit_bytes,
            witness_bytes,
            proof,
            &claimed,
            MPIConfig::prover_new(Some(universe), Some(world)),
        );
    },
    |(circuit_bytes, _), _| { circuit_bytes.len() },
    |proof, _shared| proof.bytes.len()
);

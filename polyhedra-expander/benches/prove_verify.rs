use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::get_constraints;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;
use utils::harness::{AuditStatus, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Expander,
    None,
    "sha256_mem_expander",
    utils::harness::BenchProperties::new(
        "Libra",
        "M31",         // See ./polyhedra-expander/src/bench.rs
        "GKR",         // https://eprint.iacr.org/2019/317
        Some("Orion"), // See ./polyhedra-expander/src/bench.rs
        "GKR",         // https://eprint.iacr.org/2019/317
        false,
        128,  // https://github.com/PolyhedraZK/Expander/blob/main/poly_commit/src/lib.rs#L6
        true, // Hash-based PCS (https://eprint.iacr.org/2022/1010.pdf)
        true,
        AuditStatus::NotAudited,
        None,
    ),
    {
        let universe = MPIConfig::init().expect("Failed to initialize MPI");
        let world = universe.world();
        (universe, world)
    },
    |size, _| prepare(size),
    |(circuit_bytes, witness_bytes), (universe, world)| get_constraints(
        circuit_bytes,
        witness_bytes,
        MPIConfig::prover_new(Some(universe), Some(world))
    ),
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

use spartan2_bench::{
    num_constraints, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
    SPARTAN2_BENCH_PROPERTIES,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Spartan2,
    None,
    "sha256_mem_spartan2",
    SPARTAN2_BENCH_PROPERTIES,
    |input_size| { prepare_sha256(input_size) },
    num_constraints,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size
);

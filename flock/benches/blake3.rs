use flock::{
    FLOCK_BENCH_PROPERTIES, num_constraints_blake3, prepare_blake3, preprocessing_size_blake3,
    proof_size_blake3, prove_blake3, verify_blake3,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Blake3,
    ProvingSystem::Flock,
    None,
    "blake3_mem_flock",
    FLOCK_BENCH_PROPERTIES,
    |_| None,
    prepare_blake3,
    num_constraints_blake3,
    prove_blake3,
    verify_blake3,
    preprocessing_size_blake3,
    proof_size_blake3
);

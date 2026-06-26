use flock::{
    FLOCK_BENCH_PROPERTIES, num_constraints_sha256, prepare_sha256, preprocessing_size_sha256,
    proof_size_sha256, prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Flock,
    Some("compressions"),
    "sha256_mem_flock",
    FLOCK_BENCH_PROPERTIES,
    |_| false,
    prepare_sha256,
    num_constraints_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size_sha256,
    proof_size_sha256
);

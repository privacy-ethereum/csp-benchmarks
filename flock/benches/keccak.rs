use flock::{
    FLOCK_BENCH_PROPERTIES, num_constraints_keccak, prepare_keccak, preprocessing_size_keccak,
    proof_size_keccak, prove_keccak, verify_keccak,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Flock,
    None,
    "keccak_mem_flock",
    FLOCK_BENCH_PROPERTIES,
    |_| false,
    prepare_keccak,
    num_constraints_keccak,
    prove_keccak,
    verify_keccak,
    preprocessing_size_keccak,
    proof_size_keccak
);

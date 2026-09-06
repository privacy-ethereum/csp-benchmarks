use halo2_circuits::HALO2_BENCH_PROPERTIES;
use halo2_circuits::bench::{
    keccak_prepare, num_constraints, preprocessing_size, proof_size, prove, verify,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Halo2,
    None,
    "keccak_mem_halo2",
    HALO2_BENCH_PROPERTIES,
    |_| None,
    keccak_prepare,
    num_constraints,
    prove,
    verify,
    preprocessing_size,
    proof_size
);

use halo2_circuits::HALO2_BENCH_PROPERTIES;
use halo2_circuits::bench::{
    num_constraints, preprocessing_size, proof_size, prove, sha256_prepare, verify,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Halo2,
    None,
    "sha256_mem_halo2",
    HALO2_BENCH_PROPERTIES,
    |_| None,
    sha256_prepare,
    num_constraints,
    prove,
    verify,
    preprocessing_size,
    proof_size
);

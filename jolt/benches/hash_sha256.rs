use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_hash_sha256, preprocessing_size, proof_size,
    prove_targeted, verify_targeted,
};
use utils::harness::ProvingSystem;
use utils::zkvm::HASH_SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::HashSha256,
    ProvingSystem::Jolt,
    None,
    "hash_sha256_mem_jolt",
    jolt_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv64imacCustomized, HASH_SHA256_BENCH) },
    prepare_hash_sha256,
    |_, _| 0,
    prove_targeted,
    verify_targeted,
    preprocessing_size,
    proof_size,
    execution_cycles
);

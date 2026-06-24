use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_hash_keccak, preprocessing_size, proof_size,
    prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::HASH_KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::HashKeccak,
    ProvingSystem::Jolt,
    None,
    "hash_keccak_mem_jolt",
    jolt_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv64imacCustomized, HASH_KECCAK_BENCH) },
    prepare_hash_keccak,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

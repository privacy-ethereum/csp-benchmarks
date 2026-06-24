use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_hash_blake3, preprocessing_size, proof_size,
    prove_targeted, verify_targeted,
};
use utils::harness::ProvingSystem;
use utils::zkvm::HASH_BLAKE3_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::HashBlake3,
    ProvingSystem::Jolt,
    None,
    "hash_blake3_mem_jolt",
    jolt_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv64imacCustomized, HASH_BLAKE3_BENCH) },
    prepare_hash_blake3,
    |_, _| 0,
    prove_targeted,
    verify_targeted,
    preprocessing_size,
    proof_size,
    execution_cycles
);

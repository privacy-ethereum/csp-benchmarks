use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_merkle_sha256, preprocessing_size, proof_size,
    prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::MERKLE_SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::MerkleSha256,
    ProvingSystem::Jolt,
    None,
    "merkle_sha256_mem_jolt",
    jolt_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv64imacCustomized, MERKLE_SHA256_BENCH) },
    prepare_merkle_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

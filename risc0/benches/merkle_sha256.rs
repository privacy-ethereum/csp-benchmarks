use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_merkle_sha256, preprocessing_size, proof_size, prove_targeted,
    risc0_bench_properties, verify_targeted,
};
use utils::harness::ProvingSystem;
use utils::zkvm::MERKLE_SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::MerkleSha256,
    ProvingSystem::Risc0,
    None,
    "merkle_sha256_mem_risc0",
    risc0_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv32imaCustomized, MERKLE_SHA256_BENCH) },
    prepare_merkle_sha256,
    |_, _| 0,
    prove_targeted,
    verify_targeted,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_hash_keccak, preprocessing_size, proof_size, prove_sha256,
    risc0_bench_properties, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::HASH_KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::HashKeccak,
    ProvingSystem::Risc0,
    None,
    "hash_keccak_mem_risc0",
    risc0_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv32imaCustomized, HASH_KECCAK_BENCH) },
    prepare_hash_keccak,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

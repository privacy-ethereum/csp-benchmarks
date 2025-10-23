use ere_sp1::compiler::RustRv32imaCustomized;
use sp1::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Sp1,
    None,
    "sha256_mem_sp1",
    { load_or_compile_program(&RustRv32imaCustomized, SHA256_BENCH) },
    prepare_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

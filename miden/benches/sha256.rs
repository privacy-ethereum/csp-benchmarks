use ere_miden::MIDEN_TARGET;
use miden::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Miden,
    None,
    "sha256_mem_miden",
    {
        load_or_compile_program(&MIDEN_TARGET, SHA256_BENCH)
    },
    prepare_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

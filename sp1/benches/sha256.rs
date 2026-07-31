use ere_sp1::compiler::RustRv64imaCustomized;
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
    utils::harness::BenchProperties {
        is_zkvm: true,
        ..Default::default()
    },
    |_| None,
    { load_or_compile_program(&RustRv64imaCustomized, SHA256_BENCH) },
    prepare_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

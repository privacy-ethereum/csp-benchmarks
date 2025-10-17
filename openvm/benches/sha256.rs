use ere_openvm::OPENVM_TARGET;
use openvm::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::zkvm::helpers::load_or_compile_program;
use utils::{harness::ProvingSystem, zkvm::SHA256_BENCH};

utils::__define_benchmark_harness!(
    sha256,
    utils::harness::BenchTarget::Sha256,
    ProvingSystem::OpenVM,
    None,
    "sha256_mem_openvm",
    { load_or_compile_program(&OPENVM_TARGET, SHA256_BENCH) },
    prepare_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use ere_jolt::JOLT_TARGET;
use jolt::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::{
    harness::ProvingSystem,
    zkvm::{SHA256_BENCH, helpers::load_or_compile_program},
};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Jolt,
    None,
    "sha256_mem_jolt",
    { load_or_compile_program(&JOLT_TARGET, SHA256_BENCH) },
    prepare_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use ere_nexus::compiler::RustRv32i;
use nexus::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Nexus,
    None,
    "sha256_mem_nexus",
    { load_or_compile_program(&RustRv32i, SHA256_BENCH) },
    prepare_sha256,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

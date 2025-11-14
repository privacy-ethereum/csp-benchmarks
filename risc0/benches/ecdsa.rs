use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_ecdsa, preprocessing_size, proof_size, prove_ecdsa,
    risc0_bench_properties, verify_ecdsa,
};
use utils::harness::ProvingSystem;
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Ecdsa,
    ProvingSystem::Risc0,
    None,
    "ecdsa_mem_risc0",
    risc0_bench_properties(),
    { load_or_compile_program(&RustRv32imaCustomized, ECDSA_BENCH) },
    prepare_ecdsa,
    |_, _| 0,
    prove_ecdsa,
    verify_ecdsa,
    preprocessing_size,
    proof_size,
    execution_cycles
);

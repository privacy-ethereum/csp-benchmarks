use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_keccak, preprocessing_size, proof_size, prove,
    risc0_bench_properties, verify_keccak,
};
use utils::harness::ProvingSystem;
use utils::zkvm::KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Risc0,
    None,
    "keccak_mem_risc0",
    risc0_bench_properties(),
    { load_or_compile_program(&RustRv32imaCustomized, KECCAK_BENCH) },
    prepare_keccak,
    |_, _| 0,
    prove,
    verify_keccak,
    preprocessing_size,
    proof_size,
    execution_cycles
);

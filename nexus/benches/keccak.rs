use ere_nexus::compiler::RustRv32i;
use nexus::{
    NEXUS_PROPS, execution_cycles, prepare_keccak, preprocessing_size, proof_size, prove,
    verify_keccak,
};
use utils::harness::ProvingSystem;
use utils::zkvm::KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Nexus,
    None,
    "keccak_mem_nexus",
    NEXUS_PROPS,
    { load_or_compile_program(&RustRv32i, KECCAK_BENCH) },
    prepare_keccak,
    |_, _| 0,
    prove,
    verify_keccak,
    preprocessing_size,
    proof_size,
    execution_cycles
);

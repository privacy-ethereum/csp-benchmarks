use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_keccak, preprocessing_size, proof_size, prove,
    verify_keccak,
};
use utils::harness::ProvingSystem;
use utils::zkvm::KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Jolt,
    None,
    "keccak_mem_jolt",
    jolt_bench_properties(),
    { load_or_compile_program(&RustRv64imacCustomized, KECCAK_BENCH) },
    prepare_keccak,
    |_, _| 0,
    prove,
    verify_keccak,
    preprocessing_size,
    proof_size,
    execution_cycles
);

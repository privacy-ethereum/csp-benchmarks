use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_ecdsa, preprocessing_size, proof_size,
    prove_ecdsa, verify_ecdsa,
};
use utils::harness::ProvingSystem;
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Ecdsa,
    ProvingSystem::Jolt,
    None,
    "ecdsa_mem_jolt",
    jolt_bench_properties(),
    { load_or_compile_program(&RustRv64imacCustomized, ECDSA_BENCH) },
    prepare_ecdsa,
    |_, _| 0,
    prove_ecdsa,
    verify_ecdsa,
    preprocessing_size,
    proof_size,
    execution_cycles
);

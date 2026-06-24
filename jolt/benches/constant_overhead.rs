use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_constant_overhead, preprocessing_size,
    proof_size, prove_sha256, verify_sha256,
};
use utils::harness::ProvingSystem;
use utils::zkvm::CONSTANT_OVERHEAD_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::ConstantOverhead,
    ProvingSystem::Jolt,
    None,
    "constant_overhead_mem_jolt",
    jolt_bench_properties(),
    |_| false,
    { load_or_compile_program(&RustRv64imacCustomized, CONSTANT_OVERHEAD_BENCH) },
    prepare_constant_overhead,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

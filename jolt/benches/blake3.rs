use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_blake3, preprocessing_size, proof_size,
    prove_blake3, verify_blake3,
};
use utils::harness::ProvingSystem;
use utils::zkvm::BLAKE3_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Blake3,
    ProvingSystem::Jolt,
    None,
    "blake3_mem_jolt",
    jolt_bench_properties(),
    |_| Some(utils::bench::Acceleration::Inline),
    { load_or_compile_program(&RustRv64imacCustomized, BLAKE3_BENCH) },
    prepare_blake3,
    |_, _| 0,
    prove_blake3,
    verify_blake3,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use miden::{
    execution_cycles, load_or_compile_blake3_programs, miden_bench_properties, prepare_blake3,
    preprocessing_size, proof_size, prove_blake3, verify_blake3,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Blake3,
    ProvingSystem::Miden,
    None,
    "blake3_mem_miden",
    miden_bench_properties(),
    |_| None,
    { load_or_compile_blake3_programs() },
    prepare_blake3,
    |_, _| 0,
    prove_blake3,
    verify_blake3,
    preprocessing_size,
    proof_size,
    execution_cycles
);

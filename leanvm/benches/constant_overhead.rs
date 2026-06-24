use leanvm::{
    compile_constant_overhead, execution_cycles, leanvm_bench_properties, num_constraints,
    prepare_constant_overhead, preprocessing_size, proof_size, prove_lean_bench, verify_lean_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::ConstantOverhead,
    ProvingSystem::LeanVM,
    None,
    "constant_overhead_mem_leanvm",
    leanvm_bench_properties(),
    |_| false,
    { compile_constant_overhead() },
    prepare_constant_overhead,
    num_constraints,
    prove_lean_bench,
    verify_lean_bench,
    preprocessing_size,
    proof_size,
    execution_cycles
);

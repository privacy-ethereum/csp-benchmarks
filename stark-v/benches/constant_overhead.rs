use stark_v::{
    execution_cycles, load_or_compile, prepare_constant_overhead, preprocessing_size, proof_size,
    prove_bench, stark_v_bench_properties, verify_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::ConstantOverhead,
    ProvingSystem::StarkV,
    None,
    "constant_overhead_mem_stark_v",
    stark_v_bench_properties(),
    |_| false,
    { load_or_compile("constant_overhead") },
    prepare_constant_overhead,
    |_, _| 0,
    |prepared, program| prove_bench(prepared, program),
    |prepared, proof, program| verify_bench(prepared, proof, program),
    |prepared, program| preprocessing_size(prepared, program),
    |proof, program| proof_size(proof, program),
    execution_cycles
);

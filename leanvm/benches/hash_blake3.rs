use leanvm::{
    compile_hash_blake3, execution_cycles, leanvm_bench_properties, num_constraints,
    prepare_hash_blake3, preprocessing_size, proof_size, prove_lean_bench, verify_lean_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::HashBlake3,
    ProvingSystem::LeanVM,
    None,
    "hash_blake3_mem_leanvm",
    leanvm_bench_properties(),
    |_| true,
    { compile_hash_blake3() },
    prepare_hash_blake3,
    num_constraints,
    prove_lean_bench,
    verify_lean_bench,
    preprocessing_size,
    proof_size,
    execution_cycles
);

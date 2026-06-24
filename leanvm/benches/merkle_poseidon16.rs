use leanvm::{
    compile_merkle_poseidon16, execution_cycles, leanvm_bench_properties, num_constraints,
    prepare_merkle_poseidon16, preprocessing_size, proof_size, prove_lean_bench, verify_lean_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::MerklePoseidon16,
    ProvingSystem::LeanVM,
    None,
    "merkle_poseidon16_mem_leanvm",
    leanvm_bench_properties(),
    |_| true,
    { compile_merkle_poseidon16() },
    prepare_merkle_poseidon16,
    num_constraints,
    prove_lean_bench,
    verify_lean_bench,
    preprocessing_size,
    proof_size,
    execution_cycles
);

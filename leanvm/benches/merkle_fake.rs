use leanvm::{
    compile_merkle_fake, execution_cycles, leanvm_bench_properties, num_constraints,
    prepare_merkle_fake, preprocessing_size, proof_size, prove_lean_bench, verify_lean_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::MerkleFake,
    ProvingSystem::LeanVM,
    None,
    "merkle_fake_mem_leanvm",
    leanvm_bench_properties(),
    |_| false,
    { compile_merkle_fake() },
    prepare_merkle_fake,
    num_constraints,
    prove_lean_bench,
    verify_lean_bench,
    preprocessing_size,
    proof_size,
    execution_cycles
);

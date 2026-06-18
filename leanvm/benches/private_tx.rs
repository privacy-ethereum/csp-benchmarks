use leanvm::{
    compile_private_tx, execution_cycles, leanvm_bench_properties, num_constraints,
    prepare_private_tx, preprocessing_size, proof_size, prove_private_tx, verify_private_tx,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,
    ProvingSystem::LeanVM,
    None,
    "private_tx_mem_leanvm",
    leanvm_bench_properties(),
    |_| false,
    { compile_private_tx() },
    prepare_private_tx,
    num_constraints,
    prove_private_tx,
    verify_private_tx,
    preprocessing_size,
    proof_size,
    execution_cycles
);

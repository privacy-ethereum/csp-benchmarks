use evm_sp1::{
    evm_sp1_bench_properties, execution_cycles, num_constraints, prepare_private_tx,
    preprocessing_size, proof_size, prove_private_tx, setup_private_tx, uses_precompile,
    verify_private_tx,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,
    ProvingSystem::EvmSp1,
    None,
    "private_tx_mem_evm_sp1",
    evm_sp1_bench_properties(),
    uses_precompile,
    { setup_private_tx() },
    prepare_private_tx,
    num_constraints,
    prove_private_tx,
    verify_private_tx,
    preprocessing_size,
    proof_size,
    execution_cycles
);

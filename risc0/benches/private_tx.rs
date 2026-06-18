use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_private_tx, preprocessing_size, proof_size, prove_private_tx,
    risc0_bench_properties, verify_private_tx,
};
use utils::harness::ProvingSystem;
use utils::zkvm::PRIVATE_TX_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,
    ProvingSystem::Risc0,
    None,
    "private_tx_mem_risc0",
    risc0_bench_properties(),
    |_| false,
    { load_or_compile_program(&RustRv32imaCustomized, PRIVATE_TX_BENCH) },
    prepare_private_tx,
    |_, _| 0,
    prove_private_tx,
    verify_private_tx,
    preprocessing_size,
    proof_size,
    execution_cycles
);

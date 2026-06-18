use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_private_tx, preprocessing_size, proof_size,
    prove_private_tx, verify_private_tx,
};
use utils::harness::ProvingSystem;
use utils::zkvm::PRIVATE_TX_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,
    ProvingSystem::Jolt,
    None,
    "private_tx_mem_jolt",
    jolt_bench_properties(),
    |_| false,
    { load_or_compile_program(&RustRv64imacCustomized, PRIVATE_TX_BENCH) },
    prepare_private_tx,
    |_, _| 0,
    prove_private_tx,
    verify_private_tx,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use ere_sp1::compiler::RustRv64imaCustomized;
use sp1::{
    execution_cycles, prepare_private_tx, preprocessing_size, proof_size, prove_private_tx,
    verify_private_tx,
};
use utils::harness::ProvingSystem;
use utils::zkvm::PRIVATE_TX_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,
    ProvingSystem::Sp1,
    None,
    "private_tx_mem_sp1",
    utils::harness::BenchProperties {
        is_zkvm: true,
        ..Default::default()
    },
    |_| false,
    { load_or_compile_program(&RustRv64imaCustomized, PRIVATE_TX_BENCH) },
    prepare_private_tx,
    |_, _| 0,
    prove_private_tx,
    verify_private_tx,
    preprocessing_size,
    proof_size,
    execution_cycles
);

use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_merkle_fake, preprocessing_size, proof_size, prove_targeted,
    risc0_bench_properties, verify_targeted,
};
use utils::harness::ProvingSystem;
use utils::zkvm::MERKLE_FAKE_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::MerkleFake,
    ProvingSystem::Risc0,
    None,
    "merkle_fake_mem_risc0",
    risc0_bench_properties(),
    |_| false,
    { load_or_compile_program(&RustRv32imaCustomized, MERKLE_FAKE_BENCH) },
    prepare_merkle_fake,
    |_, _| 0,
    prove_targeted,
    verify_targeted,
    preprocessing_size,
    proof_size,
    execution_cycles
);

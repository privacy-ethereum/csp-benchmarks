use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{
    execution_cycles, jolt_bench_properties, prepare_merkle_keccak, preprocessing_size, proof_size,
    prove_targeted, verify_targeted,
};
use utils::harness::ProvingSystem;
use utils::zkvm::MERKLE_KECCAK_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::MerkleKeccak,
    ProvingSystem::Jolt,
    None,
    "merkle_keccak_mem_jolt",
    jolt_bench_properties(),
    |_| true,
    { load_or_compile_program(&RustRv64imacCustomized, MERKLE_KECCAK_BENCH) },
    prepare_merkle_keccak,
    |_, _| 0,
    prove_targeted,
    verify_targeted,
    preprocessing_size,
    proof_size,
    execution_cycles
);

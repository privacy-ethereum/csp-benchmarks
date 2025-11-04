use ere_miden::compiler::MidenAsm;
use miden::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::{AuditStatus, ProvingSystem};
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Miden,
    None,
    "sha256_mem_miden",
    utils::harness::BenchProperties::new(
        "miden",
        "Goldilocks", // 2^64 - 2^32 + 1; https://0xmiden.github.io/miden-vm/design/main.html#design
        "STARK",      // https://0xmiden.github.io/miden-vm/intro/main.html
        Some("FRI"), // https://0xmiden.github.io/miden-vm/user_docs/assembly/cryptographic_operations.html#fri-folding
        "AIR", // https://0xmiden.github.io/miden-vm/design/chiplets/hasher.html?highlight=AIR#air-constraints
        true,  // https://github.com/0xPolygonMiden/miden-vm
        96, // https://0xmiden.github.io/miden-vm/intro/performance.html?highlight=security#single-core-prover-performance
        true, // hash-based PCS
        true, // https://github.com/0xPolygonMiden/miden-vm/releases
        AuditStatus::NotAudited, // https://github.com/0xPolygonMiden/miden-vm
        Some("Miden VM"), // stack-based ISA with MAST; https://hackmd.io/@bobbinth/ry-OIBwPF
    ),
    { load_or_compile_program(&MidenAsm, SHA256_BENCH) },
    prepare_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

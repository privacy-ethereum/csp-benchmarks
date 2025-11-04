use ere_risc0::compiler::RustRv32imaCustomized;
use risc0::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::{AuditStatus, ProvingSystem};
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Risc0,
    None,
    "sha256_mem_risc0",
    utils::harness::BenchProperties::new(
        "risc0",
        "BabyBear",  // 15 × 2^27 + 1; https://dev.risczero.com/proof-system-in-detail.pdf
        "STARK",     // https://dev.risczero.com/proof-system/stark-by-hand
        Some("FRI"), // https://dev.risczero.com/proof-system/stark-by-hand
        "AIR",       // https://dev.risczero.com/proof-system/proof-system-sequence-diagram
        true,        // https://dev.risczero.com/api/security-model
        96,   // 96-bit base STARK, 99-bit recursion; https://dev.risczero.com/api/security-model
        true, // STARK is PQ-safe (Groth16 compression is not); https://dev.risczero.com/api/security-model
        true, // https://github.com/risc0/risc0/releases
        AuditStatus::Audited, // https://github.com/risc0/rz-security/tree/main/audits
        Some("RISC-V RV32IM"), // base + multiplication; https://dev.risczero.com/reference-docs/about-risc-v
    ),
    { load_or_compile_program(&RustRv32imaCustomized, SHA256_BENCH) },
    prepare_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

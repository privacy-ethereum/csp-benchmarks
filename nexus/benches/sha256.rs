use ere_nexus::compiler::RustRv32i;
use nexus::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use utils::harness::{AuditStatus, ProvingSystem};
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Nexus,
    None,
    "sha256_mem_nexus",
    utils::harness::BenchProperties::new(
        "Circle STARK",
        "M31",                   // 2^31 - 1; https://specification.nexus.xyz/
        "Circle FRI",            // https://eprint.iacr.org/2024/278.pdf
        Some("Circle FRI"),      // https://eprint.iacr.org/2024/278.pdf
        "AIR",                   // https://specification.nexus.xyz/
        true,                    // https://whitepaper.nexus.xyz/
        true,                    // zkVM
        0,    // TODO: https://github.com/privacy-ethereum/csp-benchmarks/issues/147
        true, // hash-based PCS
        true, // https://github.com/nexus-xyz/nexus-zkvm/releases
        AuditStatus::NotAudited, // https://github.com/nexus-xyz/nexus-zkvm
        Some("RISC-V RV32I"), // base ISA + precompiles; https://specification.nexus.xyz/
    ),
    { load_or_compile_program(&RustRv32i, SHA256_BENCH) },
    prepare_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);

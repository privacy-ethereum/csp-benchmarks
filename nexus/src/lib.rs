use std::borrow::Cow;

use ere_nexus::{EreNexus, NexusExtension, compiler::RustRv32i};
use ere_zkvm_interface::ProverResource;
use utils::harness::{AuditStatus, BenchProperties};
use utils::zkvm::{CompiledProgram, PreparedKeccak, PreparedSha256, build_input};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove, prove_sha256, verify_keccak,
    verify_sha256,
};

pub const NEXUS_PROPS: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Circle STARK"),
    field_curve: Cow::Borrowed("M31"), // 2^31 - 1; https://specification.nexus.xyz/
    iop: Cow::Borrowed("Circle FRI"),  // https://eprint.iacr.org/2024/278.pdf
    pcs: Some(Cow::Borrowed("Circle FRI")), // https://eprint.iacr.org/2024/278.pdf
    arithm: Cow::Borrowed("AIR"),      // https://specification.nexus.xyz/
    is_zk: false,                      // Based on STWO which is currently not ZK
    is_zkvm: true,
    security_bits: 0, // TODO: https://github.com/privacy-ethereum/csp-benchmarks/issues/147
    is_pq: true,      // hash-based PCS
    is_maintained: true, // https://github.com/nexus-xyz/nexus-zkvm/releases
    is_audited: AuditStatus::NotAudited, // https://github.com/nexus-xyz/nexus-zkvm
    isa: Some(Cow::Borrowed("RISC-V RV32I")), // base ISA + precompiles; https://specification.nexus.xyz/
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<RustRv32i>,
) -> PreparedSha256<EreNexus> {
    let vm = EreNexus::new(program.program.clone(), ProverResource::Cpu).unwrap();

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_keccak(
    input_size: usize,
    program: &CompiledProgram<RustRv32i>,
) -> PreparedKeccak<EreNexus> {
    let vm = EreNexus::with_extensions(
        program.program.clone(),
        ProverResource::Cpu,
        NexusExtension::keccak_extensions().to_vec(),
    )
    .unwrap();

    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    let input = build_input(message_bytes);

    PreparedKeccak::with_expected_digest(vm, input, program.byte_size, digest)
}

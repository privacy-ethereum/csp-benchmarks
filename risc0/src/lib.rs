use bincode::Options;
use ere_risc0::{EreRisc0, compiler::RustRv32imaCustomized};
use ere_zkvm_interface::{Input, ProverResourceType};
use utils::harness::{AuditStatus, BenchProperties};
use utils::zkvm::{
    CompiledProgram, PreparedEcdsa, PreparedKeccak, PreparedSha256, encode_public_key,
};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove, prove_ecdsa, prove_sha256,
    verify_ecdsa, verify_keccak, verify_sha256,
};

pub fn risc0_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "STARK",
        "BabyBear",  // 15 × 2^27 + 1; https://dev.risczero.com/proof-system-in-detail.pdf
        "STARK",     // https://dev.risczero.com/proof-system/stark-by-hand
        Some("FRI"), // https://dev.risczero.com/proof-system/stark-by-hand
        "AIR",       // https://dev.risczero.com/proof-system/proof-system-sequence-diagram
        true,        // https://dev.risczero.com/api/security-model
        true,        // zkVM
        96,   // 96-bit base STARK, 99-bit recursion; https://dev.risczero.com/api/security-model
        true, // STARK is PQ-safe (Groth16 compression is not); https://dev.risczero.com/api/security-model
        true, // https://github.com/risc0/risc0/releases
        AuditStatus::Audited, // https://github.com/risc0/rz-security/tree/main/audits
        Some("RISC-V RV32IM"), // base + multiplication; https://dev.risczero.com/reference-docs/about-risc-v
    )
}

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedSha256<EreRisc0> {
    let vm = EreRisc0::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("failed to build risc0 prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

/// Prepares an ECDSA signature verification benchmark (single secp256k1 signature).
pub fn prepare_ecdsa(
    _input_size: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedEcdsa<EreRisc0> {
    let vm = EreRisc0::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("failed to build risc0 prover instance");

    let (digest, (pub_key_x, pub_key_y), signature) = utils::generate_ecdsa_k256_input();

    let encoded_verifying_key = encode_public_key(&pub_key_x, &pub_key_y)
        .expect("generated public key should have valid size");

    let input = build_framed_ecdsa_input(encoded_verifying_key.clone(), digest.clone(), signature);

    PreparedEcdsa::with_expected_values(
        vm,
        input,
        program.byte_size,
        (pub_key_x, pub_key_y),
        digest,
    )
}

/// Prepares a Keccak256 hash benchmark.
pub fn prepare_keccak(
    input_size: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedKeccak<EreRisc0> {
    let vm = EreRisc0::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("failed to build risc0 prover instance");

    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedKeccak::with_expected_digest(vm, input, program.byte_size, digest)
}

/// Build risc0 input with length-prefixed frame format.
fn build_framed_input(data: Vec<u8>) -> Input {
    let len = data.len() as u32;
    let mut framed = Vec::with_capacity(4 + data.len());
    framed.extend_from_slice(&len.to_le_bytes());
    framed.extend(data);
    Input::new().with_stdin(framed)
}

/// Build risc0 ECDSA input with framing.
fn build_framed_ecdsa_input(
    encoded_verifying_key: Vec<u8>,
    digest: Vec<u8>,
    signature: Vec<u8>,
) -> Input {
    let data = (encoded_verifying_key, digest, signature);
    let serialized = bincode::options()
        .serialize(&data)
        .expect("failed to serialize ECDSA input");
    build_framed_input(serialized)
}

//! Stark-V benchmark integration for csp-benchmarks.

use ere_zkvm_interface::Input;
use stark_v_sdk::{StarkV, StarkVCompiler};
use utils::harness::{AuditStatus, BenchProperties};
use utils::zkvm::{CompiledProgram, PreparedKeccak, PreparedSha256};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove, prove_sha256, verify_keccak,
    verify_sha256,
};

pub fn stark_v_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "STARK",
        "M31",
        "STARK",
        Some("FRI"),
        "AIR",
        false, // Not ZK
        true,
        96,
        true,
        true,
        AuditStatus::NotAudited,
        Some("RISC-V RV32IM"),
    )
}

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<StarkVCompiler>,
) -> PreparedSha256<StarkV> {
    let vm = StarkV::new(program.program.clone());
    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_prefixed_input(message_bytes);
    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_keccak(
    input_size: usize,
    program: &CompiledProgram<StarkVCompiler>,
) -> PreparedKeccak<StarkV> {
    let vm = StarkV::new(program.program.clone());
    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    let input = build_prefixed_input(message_bytes);
    PreparedKeccak::with_expected_digest(vm, input, program.byte_size, digest)
}

/// Build stark-v input with length-prefixed format.
///
/// The guest programs expect: [len: u32 LE][data: u8...]
fn build_prefixed_input(data: Vec<u8>) -> Input {
    Input::new().with_prefixed_stdin(data)
}

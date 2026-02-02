use ere_nexus::{EreNexus, compiler::RustRv32i};
use ere_zkvm_interface::ProverResourceType;
use utils::zkvm::{CompiledProgram, PreparedSha256, build_input};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<RustRv32i>,
) -> PreparedSha256<EreNexus> {
    let vm = EreNexus::new(program.program.clone(), ProverResourceType::Cpu).unwrap();

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

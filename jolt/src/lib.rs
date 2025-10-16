use ere_jolt::{EreJolt, JOLT_TARGET};
use utils::zkvm::{CompiledProgram, PreparedSha256, build_input};
use zkvm_interface::ProverResourceType;

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<JOLT_TARGET>,
) -> PreparedSha256<EreJolt> {
    let vm = EreJolt::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, _digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::new(vm, input, program.byte_size)
}

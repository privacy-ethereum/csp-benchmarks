use ere_sp1::{EreSP1, compiler::RustRv32imaCustomized};
use ere_zkvm_interface::ProverResourceType;
use utils::zkvm::{CompiledProgram, PreparedSha256, build_input};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedSha256<EreSP1> {
    let vm = EreSP1::new(program.program.clone(), ProverResourceType::Cpu);

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

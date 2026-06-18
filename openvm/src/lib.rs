use ere_openvm::{EreOpenVM, compiler::RustRv32imaCustomized};
use ere_zkvm_interface::ProverResource;
use utils::zkvm::{
    CompiledProgram, PreparedPrivateTx, PreparedSha256, build_input, build_private_tx_input,
};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_private_tx, prove_sha256,
    verify_private_tx, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedSha256<EreOpenVM> {
    let vm = EreOpenVM::new(program.program.clone(), ProverResource::Cpu)
        .expect("failed to build OpenVM prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_private_tx(
    depth: usize,
    program: &CompiledProgram<RustRv32imaCustomized>,
) -> PreparedPrivateTx<EreOpenVM> {
    let vm = EreOpenVM::new(program.program.clone(), ProverResource::Cpu)
        .expect("failed to build OpenVM prover instance");

    let (input_bytes, expected_public_values) = utils::generate_private_tx_input(depth);
    let input = build_private_tx_input(input_bytes);

    PreparedPrivateTx::with_expected_public_values(
        vm,
        input,
        program.byte_size,
        expected_public_values,
    )
}

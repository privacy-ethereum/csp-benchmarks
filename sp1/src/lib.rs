use ere_sp1::EreSP1;
use utils::zkvm::{CompiledProgram, PreparedSha256, build_input};
use zkvm_interface::ProverResourceType;

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF>,
) -> PreparedSha256<EreSP1> {
    let vm = EreSP1::new(program.program.clone(), ProverResourceType::Cpu);

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

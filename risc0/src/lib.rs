use ere_risc0::{EreRisc0, RV32_IM_RISC0_ZKVM_ELF};
use std::path::PathBuf;
use utils::zkvm::{
    CompiledProgram, PreparedSha256, SHA256_BENCH, build_input, compile_guest_program,
};
use zkvm_interface::ProverResourceType;

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(input_size: usize) -> PreparedSha256<EreRisc0> {
    let guest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest")
        .join(SHA256_BENCH);
    let CompiledProgram { program, byte_size } =
        compile_guest_program(&RV32_IM_RISC0_ZKVM_ELF, &guest_path)
            .expect("failed to compile risc0 guest program");

    let vm = EreRisc0::new(program, ProverResourceType::Cpu)
        .expect("failed to build risc0 prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, byte_size, digest)
}

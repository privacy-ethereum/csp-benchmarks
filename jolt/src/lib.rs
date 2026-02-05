use std::collections::HashMap;

use ere_jolt::{EreJolt, compiler::RustRv64imacCustomized};
use ere_zkvm_interface::ProverResourceType;
use utils::zkvm::{CompiledProgram, PreparedSha256, build_input};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

pub fn prepare_sha256(
    input_size: usize,
    programs: &HashMap<usize, CompiledProgram<RustRv64imacCustomized>>,
) -> PreparedSha256<EreJolt> {
    let program = &programs
        .get(&input_size)
        .expect("program not found in cache");
    let vm = EreJolt::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, _digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::new(vm, input, program.byte_size)
}

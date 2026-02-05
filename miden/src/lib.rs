use ere_miden::{EreMiden, compiler::MidenAsm};
use ere_zkvm_interface::{Input, ProverResourceType};
use std::convert::TryInto;
use utils::zkvm::{CompiledProgram, PreparedSha256, ProofArtifacts};

pub use utils::zkvm::{execution_cycles, preprocessing_size, proof_size, prove_sha256};

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<MidenAsm>,
) -> PreparedSha256<EreMiden> {
    let vm = EreMiden::new(program.program.clone(), ProverResourceType::Cpu)
        .expect("failed to build miden prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

// Miden has custom verification logic due to special public value decoding
pub fn verify_sha256(
    prepared: &PreparedSha256<EreMiden>,
    proof: &ProofArtifacts,
    _: &&CompiledProgram<MidenAsm>,
) {
    let public_values = prepared.verify(&proof.proof).expect("miden verify failed");

    assert_eq!(public_values, proof.public_values, "public values mismatch");

    let digest_bytes = decode_public_values(&proof.public_values);
    let expected_digest = prepared
        .expected_digest()
        .expect("expected digest not recorded");
    assert_eq!(digest_bytes, expected_digest, "digest mismatch");
}

fn build_input(data: Vec<u8>) -> Input {
    let len = data.len();
    let mut stdin = Vec::new();

    // Write the length as u64 LE bytes
    stdin.extend_from_slice(&(len as u64).to_le_bytes());

    let blocks = len.div_ceil(16);
    let words_needed = blocks * 4;

    let mut words: Vec<u32> = data
        .chunks(4)
        .map(|chunk| {
            let mut bytes = [0u8; 4];
            bytes[..chunk.len()].copy_from_slice(chunk);
            u32::from_be_bytes(bytes)
        })
        .collect();
    words.resize(words_needed, 0);

    for block in words.chunks_exact(4) {
        for &word in block.iter().rev() {
            stdin.extend_from_slice(&(word as u64).to_le_bytes());
        }
    }
    Input::new().with_stdin(stdin)
}

fn decode_public_values(raw: &[u8]) -> Vec<u8> {
    raw.chunks_exact(8)
        .take(8)
        .flat_map(|chunk| {
            let word =
                u64::from_le_bytes(chunk.try_into().expect("invalid miden output chunk")) as u32;
            word.to_be_bytes()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ere_zkvm_interface::zkVM;

    #[test]
    fn miden_sha256_matches_reference_digest() {
        // Build a program for tests
        use ere_miden::compiler::MidenAsm;
        use utils::zkvm::{SHA256_BENCH, compile_guest_program, guest_dir};
        let guest_path = guest_dir(SHA256_BENCH);
        let program =
            compile_guest_program(&MidenAsm, &guest_path).expect("compile guest program for tests");
        let prepared = prepare_sha256(2048, &program);

        // Execute the guest to obtain the committed digest bytes
        let (public_values, _) = prepared
            .vm()
            .execute(prepared.input())
            .expect("guest execution must succeed");
        let digest_bytes = decode_public_values(&public_values);
        let expected_digest = prepared
            .expected_digest()
            .expect("expected digest not recorded");
        assert_eq!(digest_bytes, expected_digest);

        // Ensure prove/verify plumbing also succeeds
        let proof = prove_sha256(&prepared, &program);
        verify_sha256(&prepared, &proof, &(&program));
    }
}

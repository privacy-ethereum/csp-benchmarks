use ere_miden::{EreMiden, MIDEN_TARGET};
use std::convert::TryInto;
use std::path::PathBuf;
use utils::zkvm::{
    CompiledProgram, PreparedSha256, ProofArtifacts, SHA256_BENCH, compile_guest_program,
};
use zkvm_interface::{Input, ProverResourceType};

pub use utils::zkvm::{execution_cycles, preprocessing_size, proof_size, prove_sha256};

pub fn prepare_sha256(input_size: usize) -> PreparedSha256<EreMiden> {
    let guest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest")
        .join(SHA256_BENCH);
    let CompiledProgram { program, byte_size } = compile_guest_program(&MIDEN_TARGET, &guest_path)
        .expect("failed to compile miden guest program");

    let vm = EreMiden::new(program, ProverResourceType::Cpu)
        .expect("failed to build miden prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, byte_size, digest)
}

// Miden has custom verification logic due to special public value decoding
pub fn verify_sha256(prepared: &PreparedSha256<EreMiden>, proof: &ProofArtifacts) {
    let public_values = prepared.verify(&proof.proof).expect("miden verify failed");

    assert_eq!(public_values, proof.public_values, "public values mismatch");

    let digest_bytes = decode_public_values(&proof.public_values);
    let expected_digest = prepared
        .expected_digest()
        .expect("expected digest not recorded");
    assert_eq!(digest_bytes, expected_digest, "digest mismatch");
}

fn build_input(data: Vec<u8>) -> Input {
    let mut input = Input::new();
    let len = data.len();
    input.write_bytes((len as u64).to_le_bytes().to_vec());

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
            input.write_bytes((word as u64).to_le_bytes().to_vec());
        }
    }
    input
}

fn decode_public_values(raw: &[u8]) -> Vec<u8> {
    raw.chunks_exact(8)
        .skip(1)
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
    use zkvm_interface::zkVM;

    #[test]
    fn miden_sha256_matches_reference_digest() {
        let prepared = prepare_sha256(2048);

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
        let proof = prove_sha256(&prepared);
        verify_sha256(&prepared, &proof);
    }
}

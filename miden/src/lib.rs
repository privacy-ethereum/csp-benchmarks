use ere_miden::{EreMiden, compiler::MidenAsm};
use ere_zkvm_interface::{Input, ProverResource};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use k256::{EncodedPoint, FieldBytes};
use std::collections::BTreeMap;
use std::convert::TryInto;
use utils::harness::{AuditStatus, BenchProperties};
use utils::zkvm::{CompiledProgram, PreparedBlake3, PreparedEcdsa, PreparedSha256, ProofArtifacts};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_blake3, prove_ecdsa, prove_sha256,
};

const BLAKE3_INPUT_SIZES: [usize; 5] = [128, 256, 512, 1024, 2048];

pub struct Blake3Programs {
    programs: BTreeMap<usize, CompiledProgram<MidenAsm>>,
}

impl Blake3Programs {
    fn get(&self, input_size: usize) -> &CompiledProgram<MidenAsm> {
        self.programs
            .get(&input_size)
            .unwrap_or_else(|| panic!("unsupported BLAKE3 benchmark size: {input_size}"))
    }
}

pub fn blake3_program_name(input_size: usize) -> String {
    assert!(
        BLAKE3_INPUT_SIZES.contains(&input_size),
        "unsupported BLAKE3 benchmark size"
    );
    format!("blake3_{input_size}")
}

pub fn load_or_compile_blake3_programs() -> Blake3Programs {
    let programs = BLAKE3_INPUT_SIZES
        .into_iter()
        .map(|input_size| {
            let name = blake3_program_name(input_size);
            let program = utils::zkvm::helpers::load_or_compile_program(&MidenAsm, &name);
            (input_size, program)
        })
        .collect();
    Blake3Programs { programs }
}

pub fn miden_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "STARK",
        "Goldilocks", // 2^64 - 2^32 + 1; https://0xmiden.github.io/miden-vm/design/main.html#design
        "STARK",      // https://0xmiden.github.io/miden-vm/intro/main.html
        Some("FRI"), // https://0xmiden.github.io/miden-vm/user_docs/assembly/cryptographic_operations.html#fri-folding
        "AIR", // https://0xmiden.github.io/miden-vm/design/chiplets/hasher.html?highlight=AIR#air-constraints
        false, // Not using HidingFriPcs, using TwoAdicFriPcs without hiding
        true,  // zkVM
        94, // Crites-Stewart re-estimate of the benchmark FRI config; see results/stark_fri_security_report.md
        true, // hash-based PCS
        true, // https://github.com/0xPolygonMiden/miden-vm/releases
        AuditStatus::NotAudited, // https://github.com/0xPolygonMiden/miden-vm
        Some("Miden"), // stack-based ISA with MAST; https://hackmd.io/@bobbinth/ry-OIBwPF
    )
}

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<MidenAsm>,
) -> PreparedSha256<EreMiden> {
    let vm = EreMiden::new(program.program.clone(), ProverResource::Cpu)
        .expect("failed to build miden prover instance");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_blake3(input_size: usize, programs: &Blake3Programs) -> PreparedBlake3<EreMiden> {
    prepare_blake3_with_program(input_size, programs.get(input_size))
}

pub fn prepare_blake3_with_program(
    input_size: usize,
    program: &CompiledProgram<MidenAsm>,
) -> PreparedBlake3<EreMiden> {
    assert!(
        matches!(input_size, 128 | 256 | 512 | 1024 | 2048),
        "unsupported BLAKE3 benchmark size"
    );
    let vm = EreMiden::new(program.program.clone(), ProverResource::Cpu)
        .expect("failed to build miden prover instance");

    let (message_bytes, digest) = utils::generate_blake3_input(input_size);
    let input = build_blake3_input(&message_bytes);

    PreparedBlake3::with_expected_digest(vm, input, program.byte_size, digest)
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

// BLAKE3 serializes each output word little-endian, unlike SHA-256.
fn decode_blake3_public_values(raw: &[u8]) -> Vec<u8> {
    raw.chunks_exact(8)
        .take(8)
        .flat_map(|chunk| {
            let word =
                u64::from_le_bytes(chunk.try_into().expect("invalid miden output chunk")) as u32;
            word.to_le_bytes()
        })
        .collect()
}

fn build_blake3_input(data: &[u8]) -> Input {
    assert!(
        data.len().is_multiple_of(64),
        "BLAKE3 input must contain full blocks"
    );

    let mut stdin = Vec::with_capacity(data.len() * 2);

    for block in data.chunks_exact(64) {
        let words: Vec<u32> = block
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("four-byte word")))
            .collect();

        // The advice tape is a queue. Pushing m15 first leaves m0 at stack top.
        for &word in words.iter().rev() {
            stdin.extend_from_slice(&(word as u64).to_le_bytes());
        }
    }

    Input::new().with_stdin(stdin)
}

pub fn verify_blake3<SharedState>(
    prepared: &PreparedBlake3<EreMiden>,
    proof: &ProofArtifacts,
    _: &SharedState,
) {
    let public_values = prepared.verify(&proof.proof).expect("miden verify failed");

    assert_eq!(public_values, proof.public_values, "public values mismatch");

    let digest_bytes = decode_blake3_public_values(&proof.public_values);
    let expected_digest = prepared
        .expected_digest()
        .expect("expected digest not recorded");
    assert_eq!(digest_bytes, expected_digest, "digest mismatch");
}

pub fn prepare_ecdsa(
    _input_size: usize,
    program: &CompiledProgram<MidenAsm>,
) -> Result<PreparedEcdsa<EreMiden>, &'static str> {
    let vm = EreMiden::new(program.program.clone(), ProverResource::Cpu)
        .map_err(|_| "failed to build miden prover instance")?;

    let (digest, (pub_key_x, pub_key_y), signature) = utils::generate_ecdsa_k256_input();

    let compressed_pk = compress_public_key(&pub_key_x, &pub_key_y)?;
    let recovery_id = compute_recovery_id(&digest, &signature, &pub_key_x, &pub_key_y)?;

    let mut signature_with_recovery = signature;
    signature_with_recovery.push(recovery_id);

    let input = build_ecdsa_input(&compressed_pk, &digest, &signature_with_recovery);

    Ok(PreparedEcdsa::with_expected_values(
        vm,
        input,
        program.byte_size,
        (pub_key_x, pub_key_y),
        digest,
    ))
}

pub fn verify_ecdsa(
    prepared: &PreparedEcdsa<EreMiden>,
    proof: &ProofArtifacts,
    _: &&CompiledProgram<MidenAsm>,
) -> Result<(), &'static str> {
    let public_values = prepared
        .verify(&proof.proof)
        .map_err(|_| "miden verify failed")?;
    if public_values != proof.public_values {
        return Err("public values mismatch");
    }

    let result = u64::from_le_bytes(
        proof.public_values[..8]
            .try_into()
            .map_err(|_| "invalid miden output")?,
    );
    if result != 1 {
        return Err("ECDSA verification failed in guest");
    }
    Ok(())
}

fn compress_public_key(pub_key_x: &[u8], pub_key_y: &[u8]) -> Result<Vec<u8>, &'static str> {
    let x = FieldBytes::from(coord_array(pub_key_x)?);
    let y = FieldBytes::from(coord_array(pub_key_y)?);
    Ok(EncodedPoint::from_affine_coordinates(&x, &y, true)
        .as_bytes()
        .to_vec())
}

fn compute_recovery_id(
    digest: &[u8],
    signature: &[u8],
    pub_key_x: &[u8],
    pub_key_y: &[u8],
) -> Result<u8, &'static str> {
    let sig = Signature::from_slice(signature).map_err(|_| "invalid signature")?;
    let x = FieldBytes::from(coord_array(pub_key_x)?);
    let y = FieldBytes::from(coord_array(pub_key_y)?);
    let point = EncodedPoint::from_affine_coordinates(&x, &y, false);
    let expected_vk =
        VerifyingKey::from_encoded_point(&point).map_err(|_| "invalid verifying key")?;

    for id_val in 0u8..=1 {
        let rid = RecoveryId::try_from(id_val).map_err(|_| "invalid recovery id")?;
        if let Ok(recovered_vk) = VerifyingKey::recover_from_prehash(digest, &sig, rid)
            && recovered_vk == expected_vk
        {
            return Ok(id_val);
        }
    }
    Err("could not determine recovery ID")
}

fn build_ecdsa_input(compressed_pk: &[u8], digest: &[u8], signature_with_recovery: &[u8]) -> Input {
    let mut stdin = Vec::new();
    pack_le_words(&mut stdin, compressed_pk);
    pack_le_words(&mut stdin, digest);
    pack_le_words(&mut stdin, signature_with_recovery);
    Input::new().with_stdin(stdin)
}

// Pack bytes as u32 LE values into u64 LE advice tape format, padded to word boundaries.
fn pack_le_words(stdin: &mut Vec<u8>, data: &[u8]) {
    let mut words: Vec<u32> = data
        .chunks(4)
        .map(|chunk| {
            let mut bytes = [0u8; 4];
            bytes[..chunk.len()].copy_from_slice(chunk);
            u32::from_le_bytes(bytes)
        })
        .collect();

    let words_needed = words.len().div_ceil(4) * 4;
    words.resize(words_needed, 0);

    for &w in &words {
        stdin.extend_from_slice(&(w as u64).to_le_bytes());
    }
}

/// Slice into array
fn coord_array(bytes: &[u8]) -> Result<[u8; 32], &'static str> {
    bytes.try_into().map_err(|_| "coordinate must be 32 bytes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ere_zkvm_interface::zkVM;

    #[test]
    fn compressed_key_is_valid_sec1() {
        let (_digest, (pub_key_x, pub_key_y), _signature) = utils::generate_ecdsa_k256_input();
        let compressed_pk = compress_public_key(&pub_key_x, &pub_key_y).unwrap();

        assert_eq!(compressed_pk.len(), 33);
        assert!(
            compressed_pk[0] == 0x02 || compressed_pk[0] == 0x03,
            "invalid prefix: 0x{:02x}",
            compressed_pk[0]
        );

        k256::PublicKey::from_sec1_bytes(&compressed_pk).expect("not valid SEC1");
    }

    #[test]
    fn miden_ecdsa_guest_executes() {
        use utils::zkvm::{ECDSA_BENCH, compile_guest_program, guest_dir};
        let guest_path = guest_dir(ECDSA_BENCH);
        let program = compile_guest_program(&MidenAsm, &guest_path).expect("compile ecdsa guest");
        let prepared = prepare_ecdsa(1, &program).unwrap();

        let (public_values, _) = prepared
            .vm()
            .execute(prepared.input())
            .expect("ecdsa guest execution must succeed");

        let result = u64::from_le_bytes(public_values[..8].try_into().unwrap());
        assert_eq!(result, 1, "ECDSA verification should return 1");
    }

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

    #[test]
    fn miden_blake3_matches_reference_digest() {
        let programs = load_or_compile_blake3_programs();

        for input_size in [128, 256, 512, 1024, 2048] {
            let prepared = prepare_blake3(input_size, &programs);
            let (public_values, _) = prepared
                .vm()
                .execute(prepared.input())
                .expect("BLAKE3 guest execution must succeed");
            let digest_bytes = decode_blake3_public_values(&public_values);
            assert_eq!(
                digest_bytes,
                prepared.expected_digest().expect("expected digest")
            );
        }

        // A small proof exercises the same compression AIR and proof plumbing;
        // execution above separately covers the two-chunk 2048-byte route.
        let prepared = prepare_blake3(128, &programs);
        let proof = prove_blake3(&prepared, &programs);
        verify_blake3(&prepared, &proof, &programs);
    }

    #[test]
    #[ignore = "proves the largest Miden BLAKE3 benchmark input"]
    fn miden_blake3_2048_proof_roundtrip() {
        let programs = load_or_compile_blake3_programs();
        let prepared = prepare_blake3(2048, &programs);
        let proof = prove_blake3(&prepared, &programs);
        verify_blake3(&prepared, &proof, &programs);
    }
}

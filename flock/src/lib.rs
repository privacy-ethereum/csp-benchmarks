use std::borrow::Cow;

use flock_prover::challenger::FsChallenger;
use flock_prover::pcs::PcsParams;
use flock_prover::proof::R1csProof;
use flock_prover::proof_io::{HashKind, R1csProofBundle};
use flock_prover::r1cs::BlockR1cs;
use flock_prover::r1cs_hashes::keccak::{KeccakSetup, STATE_BITS, State, keccak_f, state_idx};
use flock_prover::r1cs_hashes::sha2::{Compression, SHA256_IV, Sha256HybridSetup, sha256_compress};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use serde::Serialize;
use utils::harness::{AuditStatus, BenchProperties};

const TRANSCRIPT_DOMAIN: &[u8] = b"csp-benchmarks-flock-hash-v0";
const SHA256_BLOCK_BYTES: usize = 64;
const SHA256_LEN_BYTES: usize = 8;
const KECCAK256_RATE_BYTES: usize = 136;
// Deliberately conservative: one full core over-approximates the true fixed
// padding/wrapper constraint cost rather than trying to model it precisely.
const FIXED_HASH_OVERHEAD_OPS: usize = 1;

pub const FLOCK_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Flock"),
    field_curve: Cow::Borrowed("F2^128"),
    iop: Cow::Borrowed("Flock"),
    pcs: Some(Cow::Borrowed("BaseFold")),
    arithm: Cow::Borrowed("R1CS"),
    is_zk: false,
    is_zkvm: false,
    security_bits: 100,
    is_pq: true,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

#[derive(Clone)]
pub struct PreparedSha256 {
    setup: Sha256HybridSetup,
    blocks: Vec<Compression>,
    operation_count: usize,
}

pub struct Sha256Proof {
    proof: R1csProof,
    commitment: flock_prover::pcs::Commitment,
}

#[derive(Clone)]
pub struct PreparedKeccak {
    setup: KeccakSetup,
    states: Vec<State>,
    operation_count: usize,
}

pub struct KeccakProof {
    proof: R1csProof,
    commitment: flock_prover::pcs::Commitment,
}

pub fn sha256_compression_count(input_size: usize) -> usize {
    (input_size + 1 + SHA256_LEN_BYTES).div_ceil(SHA256_BLOCK_BYTES)
}

pub fn sha256_operation_count(input_size: usize) -> usize {
    sha256_compression_count(input_size) + FIXED_HASH_OVERHEAD_OPS
}

pub fn keccak256_permutation_count(input_size: usize) -> usize {
    input_size / KECCAK256_RATE_BYTES + 1
}

pub fn keccak_operation_count(input_size: usize) -> usize {
    keccak256_permutation_count(input_size) + FIXED_HASH_OVERHEAD_OPS
}

fn build_sha256_compressions(input_size: usize) -> Vec<Compression> {
    let (message, expected_digest) = utils::generate_sha256_input(input_size);
    let mut padded = message;
    let bit_len = (input_size as u64) * 8;

    padded.push(0x80);
    while padded.len() % SHA256_BLOCK_BYTES != SHA256_BLOCK_BYTES - SHA256_LEN_BYTES {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    assert_eq!(
        padded.len() / SHA256_BLOCK_BYTES,
        sha256_compression_count(input_size)
    );

    let mut cv = SHA256_IV;
    let mut blocks = Vec::with_capacity(sha256_operation_count(input_size));

    for chunk in padded.chunks_exact(SHA256_BLOCK_BYTES) {
        let message_block: [u32; 16] = std::array::from_fn(|i| {
            let start = i * 4;
            u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ])
        });
        blocks.push((cv, message_block));
        cv = sha256_compress(&cv, &message_block);
    }

    let digest: Vec<u8> = cv.iter().flat_map(|word| word.to_be_bytes()).collect();
    assert_eq!(digest, expected_digest);

    let mut rng = StdRng::seed_from_u64(input_size as u64);
    let overhead_block: [u32; 16] = std::array::from_fn(|_| rng.next_u32());
    blocks.push((cv, overhead_block));
    assert_eq!(blocks.len(), sha256_operation_count(input_size));

    blocks
}

fn keccak_state_pos(byte_idx: usize, bit_idx: usize) -> usize {
    let lane = byte_idx / 8;
    let lane_byte = byte_idx % 8;
    let x = lane % 5;
    let y = lane / 5;
    let z = lane_byte * 8 + bit_idx;
    state_idx(x, y, z)
}

fn absorb_keccak_block(state: &mut State, block: &[u8; KECCAK256_RATE_BYTES]) {
    for (byte_idx, byte) in block.iter().enumerate() {
        for bit_idx in 0..8 {
            if (byte >> bit_idx) & 1 == 1 {
                let pos = keccak_state_pos(byte_idx, bit_idx);
                state[pos] ^= true;
            }
        }
    }
}

fn squeeze_keccak_prefix(state: &State, output_len: usize) -> Vec<u8> {
    (0..output_len)
        .map(|byte_idx| {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                if state[keccak_state_pos(byte_idx, bit_idx)] {
                    byte |= 1 << bit_idx;
                }
            }
            byte
        })
        .collect()
}

fn build_keccak_permutations(input_size: usize) -> Vec<State> {
    let (message, expected_digest) = utils::generate_keccak_input(input_size);
    let mut state = [false; STATE_BITS];
    let mut states = Vec::with_capacity(keccak_operation_count(input_size));

    let full_blocks = input_size / KECCAK256_RATE_BYTES;
    for chunk in message.chunks_exact(KECCAK256_RATE_BYTES).take(full_blocks) {
        let block: &[u8; KECCAK256_RATE_BYTES] = chunk.try_into().unwrap();
        absorb_keccak_block(&mut state, block);
        states.push(state);
        keccak_f(&mut state);
    }

    let remainder = &message[full_blocks * KECCAK256_RATE_BYTES..];
    let mut final_block = [0u8; KECCAK256_RATE_BYTES];
    final_block[..remainder.len()].copy_from_slice(remainder);
    final_block[remainder.len()] ^= 0x01;
    final_block[KECCAK256_RATE_BYTES - 1] ^= 0x80;
    absorb_keccak_block(&mut state, &final_block);
    states.push(state);
    keccak_f(&mut state);

    let digest = squeeze_keccak_prefix(&state, expected_digest.len());
    assert_eq!(digest, expected_digest);

    states.push(state);
    assert_eq!(states.len(), keccak_operation_count(input_size));

    states
}

pub fn prepare_sha256(input_size: usize) -> PreparedSha256 {
    let _ = flock_prover::init_perf_thread_pool();
    let blocks = build_sha256_compressions(input_size);
    let operation_count = blocks.len();
    let setup = Sha256HybridSetup::new(operation_count);

    PreparedSha256 {
        setup,
        blocks,
        operation_count,
    }
}

pub fn prove_sha256(prepared: &PreparedSha256) -> Sha256Proof {
    let mut challenger = FsChallenger::new(TRANSCRIPT_DOMAIN);
    let (proof, commitment, _claim) = prepared
        .setup
        .prove_fast_basefold(&prepared.blocks, &mut challenger);

    Sha256Proof { proof, commitment }
}

pub fn verify_sha256(prepared: &PreparedSha256, proof: &Sha256Proof) {
    let mut challenger = FsChallenger::new(TRANSCRIPT_DOMAIN);
    prepared
        .setup
        .verify_basefold(&proof.commitment, &proof.proof, &mut challenger)
        .expect("Flock SHA-256 proof must verify");
}

pub fn prepare_keccak(input_size: usize) -> PreparedKeccak {
    let _ = flock_prover::init_perf_thread_pool();
    let states = build_keccak_permutations(input_size);
    let operation_count = states.len();
    let setup = KeccakSetup::new(operation_count);

    PreparedKeccak {
        setup,
        states,
        operation_count,
    }
}

pub fn prove_keccak(prepared: &PreparedKeccak) -> KeccakProof {
    let mut challenger = FsChallenger::new(TRANSCRIPT_DOMAIN);
    let (proof, commitment, _claim) = prepared
        .setup
        .prove_fast_basefold(&prepared.states, &mut challenger);

    KeccakProof { proof, commitment }
}

pub fn verify_keccak(prepared: &PreparedKeccak, proof: &KeccakProof) {
    let mut challenger = FsChallenger::new(TRANSCRIPT_DOMAIN);
    prepared
        .setup
        .verify_basefold(&proof.commitment, &proof.proof, &mut challenger)
        .expect("Flock Keccak proof must verify");
}

pub fn num_constraints_sha256(prepared: &PreparedSha256) -> usize {
    prepared.setup.r1cs.useful_bits * prepared.setup.n_block_slots()
}

pub fn num_constraints_keccak(prepared: &PreparedKeccak) -> usize {
    prepared.setup.r1cs.useful_bits * prepared.setup.n_keccak_slots()
}

#[derive(Serialize)]
struct PreprocessingDescriptor<'a> {
    hash_kind: HashKind,
    m: usize,
    k_log: usize,
    k_skip: usize,
    useful_bits: usize,
    operation_count: usize,
    n_slots: usize,
    pcs_params: &'a PcsParams,
    statement_digest: [u8; 32],
}

fn preprocessing_size(
    hash_kind: HashKind,
    r1cs: &BlockR1cs,
    params: &PcsParams,
    operation_count: usize,
) -> usize {
    // Flock's hash circuits are hardcoded and this path does not persist a
    // proving key, so record the stable public setup descriptor consumed by
    // the verifier rather than witness/prover scratch state.
    let descriptor = PreprocessingDescriptor {
        hash_kind,
        m: r1cs.m,
        k_log: r1cs.k_log,
        k_skip: r1cs.k_skip,
        useful_bits: r1cs.useful_bits,
        operation_count,
        n_slots: r1cs.n_outer(),
        pcs_params: params,
        statement_digest: r1cs.statement_digest(),
    };

    bincode::serialize(&descriptor)
        .expect("Flock preprocessing descriptor must serialize")
        .len()
}

pub fn preprocessing_size_sha256(prepared: &PreparedSha256) -> usize {
    preprocessing_size(
        HashKind::Sha2,
        &prepared.setup.r1cs,
        &prepared.setup.pcs_params,
        prepared.operation_count,
    )
}

pub fn preprocessing_size_keccak(prepared: &PreparedKeccak) -> usize {
    preprocessing_size(
        HashKind::Keccak,
        &prepared.setup.r1cs,
        &prepared.setup.pcs_params,
        prepared.operation_count,
    )
}

pub fn proof_size_sha256(proof: &Sha256Proof) -> usize {
    R1csProofBundle {
        commitment: proof.commitment.clone(),
        proof: proof.proof.clone(),
    }
    .to_bytes()
    .len()
}

pub fn proof_size_keccak(proof: &KeccakProof) -> usize {
    R1csProofBundle {
        commitment: proof.commitment.clone(),
        proof: proof.proof.clone(),
    }
    .to_bytes()
    .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduced_sha256_roundtrip() {
        let prepared = prepare_sha256(128);
        let proof = prove_sha256(&prepared);
        verify_sha256(&prepared, &proof);
        assert!(proof_size_sha256(&proof) > 0);
        assert_eq!(prepared.operation_count, 4);
        assert_eq!(
            num_constraints_sha256(&prepared),
            prepared.setup.r1cs.useful_bits * prepared.setup.n_block_slots()
        );
    }

    #[test]
    fn reduced_keccak_roundtrip() {
        let prepared = prepare_keccak(128);
        let proof = prove_keccak(&prepared);
        verify_keccak(&prepared, &proof);
        assert!(proof_size_keccak(&proof) > 0);
        assert_eq!(prepared.operation_count, 2);
        assert_eq!(
            num_constraints_keccak(&prepared),
            prepared.setup.r1cs.useful_bits * prepared.setup.n_keccak_slots()
        );
    }

    #[test]
    fn byte_size_maps_to_hash_operations_with_fixed_overhead() {
        assert_eq!(sha256_compression_count(128), 3);
        assert_eq!(sha256_operation_count(128), 4);
        assert_eq!(sha256_compression_count(2048), 33);
        assert_eq!(sha256_operation_count(2048), 34);

        assert_eq!(keccak256_permutation_count(128), 1);
        assert_eq!(keccak_operation_count(128), 2);
        assert_eq!(keccak256_permutation_count(2048), 16);
        assert_eq!(keccak_operation_count(2048), 17);
    }
}

use std::borrow::Cow;

use flock_prover::pcs::PcsParams;
use flock_prover::proof_io::HashKind;
use flock_prover::r1cs::{BlockR1cs, SparseBinaryMatrix, WitnessLayout};
use serde::Serialize;
use utils::harness::{AuditStatus, BenchProperties};

mod full_blake3;
mod full_keccak;
mod full_sha256;

const TRANSCRIPT_DOMAIN: &[u8] = b"csp-benchmarks-flock-hash-v0";
const SHA256_BLOCK_BYTES: usize = 64;
const SHA256_LEN_BYTES: usize = 8;
const KECCAK256_RATE_BYTES: usize = 136;
const BLAKE3_BLOCK_BYTES: usize = 64;
const BLAKE3_CHUNK_BYTES: usize = 1024;

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

pub struct PreparedSha256 {
    setup: full_sha256::FullSha256Setup,
    operation_count: usize,
}

pub struct Sha256Proof {
    proof: full_sha256::FullSha256Proof,
}

pub struct PreparedKeccak {
    setup: full_keccak::FullKeccakSetup,
    operation_count: usize,
}

pub struct KeccakProof {
    proof: full_keccak::FullKeccakProof,
}

pub struct PreparedBlake3 {
    setup: full_blake3::FullBlake3Setup,
    operation_count: usize,
}

pub struct Blake3Proof {
    proof: full_blake3::FullBlake3Proof,
}

pub fn sha256_compression_count(input_size: usize) -> usize {
    (input_size + 1 + SHA256_LEN_BYTES).div_ceil(SHA256_BLOCK_BYTES)
}

pub fn sha256_operation_count(input_size: usize) -> usize {
    sha256_compression_count(input_size)
}

pub fn keccak256_permutation_count(input_size: usize) -> usize {
    input_size / KECCAK256_RATE_BYTES + 1
}

pub fn keccak_operation_count(input_size: usize) -> usize {
    keccak256_permutation_count(input_size)
}

pub fn blake3_compression_count(input_size: usize) -> usize {
    assert!(
        input_size.is_multiple_of(BLAKE3_BLOCK_BYTES) && input_size <= 2 * BLAKE3_CHUNK_BYTES,
        "BLAKE3 benchmark supports full blocks through 2048 bytes"
    );
    input_size / BLAKE3_BLOCK_BYTES + usize::from(input_size > BLAKE3_CHUNK_BYTES)
}

pub fn blake3_operation_count(input_size: usize) -> usize {
    blake3_compression_count(input_size)
}

pub fn prepare_sha256(input_size: usize) -> PreparedSha256 {
    let setup = full_sha256::prepare(input_size);
    let operation_count = setup.n_compressions;

    PreparedSha256 {
        setup,
        operation_count,
    }
}

pub fn prove_sha256(prepared: &PreparedSha256) -> Sha256Proof {
    Sha256Proof {
        proof: full_sha256::prove(&prepared.setup, TRANSCRIPT_DOMAIN),
    }
}

pub fn verify_sha256(prepared: &PreparedSha256, proof: &Sha256Proof) {
    full_sha256::verify(&prepared.setup, &proof.proof, TRANSCRIPT_DOMAIN);
}

pub fn prepare_keccak(input_size: usize) -> PreparedKeccak {
    let setup = full_keccak::prepare(input_size);
    let operation_count = setup.n_permutations;

    PreparedKeccak {
        setup,
        operation_count,
    }
}

pub fn prove_keccak(prepared: &PreparedKeccak) -> KeccakProof {
    KeccakProof {
        proof: full_keccak::prove(&prepared.setup, TRANSCRIPT_DOMAIN),
    }
}

pub fn verify_keccak(prepared: &PreparedKeccak, proof: &KeccakProof) {
    full_keccak::verify(&prepared.setup, &proof.proof, TRANSCRIPT_DOMAIN);
}

pub fn prepare_blake3(input_size: usize) -> PreparedBlake3 {
    let setup = full_blake3::prepare(input_size, false);
    let operation_count = setup.n_compressions;

    PreparedBlake3 {
        setup,
        operation_count,
    }
}

pub fn prove_blake3(prepared: &PreparedBlake3) -> Blake3Proof {
    Blake3Proof {
        proof: full_blake3::prove(&prepared.setup, TRANSCRIPT_DOMAIN),
    }
}

pub fn verify_blake3(prepared: &PreparedBlake3, proof: &Blake3Proof) {
    full_blake3::verify(&prepared.setup, &proof.proof, TRANSCRIPT_DOMAIN);
}

pub fn num_constraints_sha256(prepared: &PreparedSha256) -> usize {
    full_sha256::num_constraints(&prepared.setup)
}

pub fn num_constraints_keccak(prepared: &PreparedKeccak) -> usize {
    full_keccak::num_constraints(&prepared.setup)
}

pub fn num_constraints_blake3(prepared: &PreparedBlake3) -> usize {
    full_blake3::num_constraints(&prepared.setup)
}

#[derive(Serialize)]
struct PreprocessingArtifact<'a> {
    hash_kind: HashKind,
    operation_count: usize,
    n_slots: usize,
    r1cs: SerializableBlockR1cs<'a>,
    pcs_params: &'a PcsParams,
    statement_digest: [u8; 32],
}

#[derive(Serialize)]
struct SerializableBlockR1cs<'a> {
    m: usize,
    k_log: usize,
    k_skip: usize,
    useful_bits: usize,
    layout: u8,
    a_0: SerializableSparseBinaryMatrix<'a>,
    b_0: SerializableSparseBinaryMatrix<'a>,
    c_0: SerializableSparseBinaryMatrix<'a>,
    const_pin: Option<usize>,
}

#[derive(Serialize)]
struct SerializableSparseBinaryMatrix<'a> {
    num_rows: usize,
    num_cols: usize,
    rows: &'a [Vec<usize>],
}

fn preprocessing_size(
    hash_kind: HashKind,
    r1cs: &BlockR1cs,
    params: &PcsParams,
    operation_count: usize,
) -> usize {
    // Flock regenerates its proving data from public setup. Report the
    // serialized setup artifact, including the materialized BlockR1cs where
    // upstream exposes one, rather than only a tiny regeneration descriptor.
    let artifact = PreprocessingArtifact {
        hash_kind,
        operation_count,
        n_slots: r1cs.n_outer(),
        r1cs: SerializableBlockR1cs::from(r1cs),
        pcs_params: params,
        statement_digest: r1cs.statement_digest(),
    };

    bincode::serialize(&artifact)
        .expect("Flock preprocessing artifact must serialize")
        .len()
}

impl<'a> From<&'a BlockR1cs> for SerializableBlockR1cs<'a> {
    fn from(r1cs: &'a BlockR1cs) -> Self {
        Self {
            m: r1cs.m,
            k_log: r1cs.k_log,
            k_skip: r1cs.k_skip,
            useful_bits: r1cs.useful_bits,
            layout: match r1cs.layout {
                WitnessLayout::RowMajor => 0,
                WitnessLayout::BatchMajor => 1,
            },
            a_0: SerializableSparseBinaryMatrix::from(&r1cs.a_0),
            b_0: SerializableSparseBinaryMatrix::from(&r1cs.b_0),
            c_0: SerializableSparseBinaryMatrix::from(&r1cs.c_0),
            const_pin: r1cs.const_pin,
        }
    }
}

impl<'a> From<&'a SparseBinaryMatrix> for SerializableSparseBinaryMatrix<'a> {
    fn from(matrix: &'a SparseBinaryMatrix) -> Self {
        Self {
            num_rows: matrix.num_rows,
            num_cols: matrix.num_cols,
            rows: &matrix.rows,
        }
    }
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

pub fn preprocessing_size_blake3(prepared: &PreparedBlake3) -> usize {
    preprocessing_size(
        HashKind::Blake3,
        &prepared.setup.r1cs,
        &prepared.setup.pcs_params,
        prepared.operation_count,
    )
}

pub fn proof_size_sha256(proof: &Sha256Proof) -> usize {
    full_sha256::proof_size(&proof.proof)
}

pub fn proof_size_keccak(proof: &KeccakProof) -> usize {
    full_keccak::proof_size(&proof.proof)
}

pub fn proof_size_blake3(proof: &Blake3Proof) -> usize {
    full_blake3::proof_size(&proof.proof)
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
        assert_eq!(prepared.operation_count, 3);
        assert_eq!(
            num_constraints_sha256(&prepared),
            prepared.setup.r1cs.useful_bits * prepared.setup.r1cs.n_outer()
        );
    }

    #[test]
    fn reduced_keccak_roundtrip() {
        let prepared = prepare_keccak(128);
        let proof = prove_keccak(&prepared);
        verify_keccak(&prepared, &proof);
        assert!(proof_size_keccak(&proof) > 0);
        assert_eq!(prepared.operation_count, 1);
        assert_eq!(
            num_constraints_keccak(&prepared),
            prepared.setup.r1cs.useful_bits * prepared.setup.r1cs.n_outer()
        );
    }

    #[test]
    fn reduced_blake3_roundtrip() {
        let prepared = prepare_blake3(128);
        let proof = prove_blake3(&prepared);
        verify_blake3(&prepared, &proof);
        assert!(proof_size_blake3(&proof) > 0);
        assert_eq!(prepared.operation_count, 2);
        assert_eq!(
            num_constraints_blake3(&prepared),
            prepared.setup.r1cs.useful_bits * prepared.setup.r1cs.n_outer()
        );
    }

    #[test]
    #[ignore = "covers the largest BLAKE3 benchmark circuit"]
    fn blake3_two_chunk_roundtrip() {
        let prepared = prepare_blake3(2048);
        let proof = prove_blake3(&prepared);
        verify_blake3(&prepared, &proof);
        assert_eq!(prepared.operation_count, 33);
    }

    #[test]
    fn byte_size_maps_to_hash_operations() {
        assert_eq!(sha256_compression_count(128), 3);
        assert_eq!(sha256_operation_count(128), 3);
        assert_eq!(sha256_compression_count(2048), 33);
        assert_eq!(sha256_operation_count(2048), 33);

        assert_eq!(keccak256_permutation_count(128), 1);
        assert_eq!(keccak_operation_count(128), 1);
        assert_eq!(keccak256_permutation_count(2048), 16);
        assert_eq!(keccak_operation_count(2048), 16);

        assert_eq!(blake3_compression_count(128), 2);
        assert_eq!(blake3_operation_count(128), 2);
        assert_eq!(blake3_compression_count(1024), 16);
        assert_eq!(blake3_compression_count(2048), 33);
        assert_eq!(blake3_operation_count(2048), 33);
    }

    #[test]
    fn full_hash_wrappers_keep_upstream_core_layout() {
        let sha = prepare_sha256(2048);
        assert_eq!(sha.setup.r1cs.k_log, flock_prover::r1cs_hashes::sha2::K_LOG);
        assert_eq!(sha.operation_count, 33);
        assert_eq!(sha.setup.r1cs.n_outer(), 64);

        let keccak = prepare_keccak(2048);
        assert_eq!(
            keccak.setup.r1cs.k_log,
            flock_prover::r1cs_hashes::keccak::K_LOG
        );
        assert_eq!(keccak.operation_count, 16);
        assert_eq!(keccak.setup.r1cs.n_outer(), 16);

        let blake3 = prepare_blake3(2048);
        assert_eq!(
            blake3.setup.r1cs.k_log,
            flock_prover::r1cs_hashes::blake3::K_LOG
        );
        assert_eq!(blake3.operation_count, 33);
        assert_eq!(blake3.setup.r1cs.n_outer(), 64);
    }
}

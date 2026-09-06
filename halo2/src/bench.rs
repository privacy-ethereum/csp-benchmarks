// `zkevm_hashes` re-exports `halo2_proofs` privately, so halo2 types come from halo2_base`
use halo2_base::{
    halo2_proofs::{
        SerdeFormat,
        halo2curves::bn256::{Bn256, Fr, G1Affine},
        plonk::{Circuit, ProvingKey, create_proof, keygen_pk, keygen_vk, verify_proof},
        poly::{
            commitment::ParamsProver,
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::{ProverSHPLONK, VerifierSHPLONK},
                strategy::SingleStrategy,
            },
        },
        transcript::{
            Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
        },
    },
    utils::fs::gen_srs,
};
use rand_core::OsRng;
use zkevm_hashes::{
    keccak::vanilla::{
        KeccakConfigParams,
        keccak_packed_multi::{get_keccak_capacity, get_num_keccak_f},
        param::{NUM_ROUNDS as KECCAK_NUM_ROUNDS, NUM_WORDS_TO_ABSORB as KECCAK_WORDS_TO_ABSORB},
    },
    sha256::vanilla::{param::SHA256_NUM_ROWS, util::get_num_sha2_blocks},
};

use crate::circuits::{KeccakCircuit, Sha256BitCircuit};

/// Rows at the end of the domain that halo2 reserves for blinding factors and so
/// cannot hold witness data. Matches the allowance used by the upstream tests.
const BLINDING_ROWS: usize = 109;

/// Below this degree the fixed-column layout of these circuits does not fit.
const MIN_K: u32 = 10;

/// Rows per keccak_f round. Trades circuit width against height; 28 is the value
/// upstream benchmarks use for the larger of their two test configurations.
const KECCAK_ROWS_PER_ROUND: usize = 28;

/// A keygen'd circuit ready to be proved.
pub struct Prepared<C: Circuit<Fr> + Clone> {
    params: ParamsKZG<Bn256>,
    pk: ProvingKey<G1Affine>,
    circuit: C,
    /// Rows the hash actually occupies, excluding padding up to `2^k`.
    used_rows: usize,
}

/// Usable rows in a degree-`k` circuit.
fn usable_rows(k: u32) -> usize {
    (1usize << k) - BLINDING_ROWS
}

/// Smallest degree whose usable rows satisfy `fits`.
fn smallest_k(fits: impl Fn(usize) -> bool) -> u32 {
    (MIN_K..=25)
        .find(|&k| fits(usable_rows(k)))
        .expect("input does not fit in a circuit of degree <= 25")
}

/// Degree and used-row count for a SHA-256 circuit over `input_size` bytes.
pub fn sha256_dimensions(input_size: usize) -> (u32, usize) {
    let rows = get_num_sha2_blocks(input_size) * SHA256_NUM_ROWS;
    (smallest_k(|usable| usable >= rows), rows)
}

/// Degree and used-row count for a Keccak circuit over `input_size` bytes.
pub fn keccak_dimensions(input_size: usize) -> (u32, usize) {
    let permutations = get_num_keccak_f(input_size);
    let k = smallest_k(|usable| get_keccak_capacity(usable, KECCAK_ROWS_PER_ROUND) >= permutations);
    // Inverse of `get_keccak_capacity`: a dummy round, the absorb lookahead window,
    // and `NUM_ROUNDS + 1` rounds per permutation, all scaled by rows per round.
    let rows = (1 + KECCAK_WORDS_TO_ABSORB + permutations * (KECCAK_NUM_ROUNDS + 1))
        * KECCAK_ROWS_PER_ROUND;
    (k, rows)
}

fn keygen<C: Circuit<Fr> + Clone>(k: u32, circuit: C, used_rows: usize) -> Prepared<C> {
    let params = gen_srs(k);
    let shape = circuit.without_witnesses();
    let vk = keygen_vk(&params, &shape).expect("vk generation failed");
    let pk = keygen_pk(&params, vk, &shape).expect("pk generation failed");
    Prepared {
        params,
        pk,
        circuit,
        used_rows,
    }
}

pub fn sha256_prepare(input_size: usize) -> Prepared<Sha256BitCircuit> {
    let (msg, _digest) = utils::generate_sha256_input(input_size);
    let (k, used_rows) = sha256_dimensions(input_size);
    keygen(
        k,
        Sha256BitCircuit::new(usable_rows(k), vec![msg]),
        used_rows,
    )
}

pub fn keccak_prepare(input_size: usize) -> Prepared<KeccakCircuit> {
    let (msg, _digest) = utils::generate_keccak_input(input_size);
    let (k, used_rows) = keccak_dimensions(input_size);
    let config = KeccakConfigParams {
        k,
        rows_per_round: KECCAK_ROWS_PER_ROUND,
    };
    keygen(
        k,
        KeccakCircuit::new(config, usable_rows(k), vec![msg]),
        used_rows,
    )
}

pub fn prove<C: Circuit<Fr> + Clone>(prepared: &Prepared<C>) -> Vec<u8> {
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<'_, Bn256>,
        Challenge255<_>,
        _,
        Blake2bWrite<Vec<u8>, G1Affine, _>,
        _,
    >(
        &prepared.params,
        &prepared.pk,
        std::slice::from_ref(&prepared.circuit),
        &[&[]],
        OsRng,
        &mut transcript,
    )
    .expect("proving failed");
    transcript.finalize()
}

// The harness types these closures as `FnMut(&PreparedContext, &Proof)` and
// `FnMut(&Proof) -> usize` with `Proof = Vec<u8>`, so the parameter must be
// `&Vec<u8>` exactly; `&[u8]` does not satisfy the bound.
#[allow(clippy::ptr_arg)]
pub fn verify<C: Circuit<Fr> + Clone>(prepared: &Prepared<C>, proof: &Vec<u8>) {
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, Bn256>,
    >(
        prepared.params.verifier_params(),
        prepared.pk.get_vk(),
        SingleStrategy::new(&prepared.params),
        &[&[]],
        &mut transcript,
    )
    .expect("verification failed");
}

/// Rows the hash occupies. This is halo2's analogue of a gate count: each row is one
/// instance of the circuit's custom gates.
pub fn num_constraints<C: Circuit<Fr> + Clone>(prepared: &Prepared<C>) -> usize {
    prepared.used_rows
}

/// Serialized proving key size. The KZG SRS is universal rather than circuit-specific,
/// so it is not counted here.
pub fn preprocessing_size<C: Circuit<Fr> + Clone>(prepared: &Prepared<C>) -> usize {
    prepared.pk.to_bytes(SerdeFormat::RawBytes).len()
}

#[allow(clippy::ptr_arg)]
pub fn proof_size(proof: &Vec<u8>) -> usize {
    proof.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_fit_and_are_minimal() {
        for size in utils::metadata::selected_byte_inputs() {
            let (k, rows) = sha256_dimensions(size);
            assert!(usable_rows(k) >= rows, "sha256 {size}: k={k} too small");
            assert!(
                k == MIN_K || usable_rows(k - 1) < rows,
                "sha256 {size}: k={k} larger than needed"
            );

            let (k, _) = keccak_dimensions(size);
            let permutations = get_num_keccak_f(size);
            assert!(
                get_keccak_capacity(usable_rows(k), KECCAK_ROWS_PER_ROUND) >= permutations,
                "keccak {size}: k={k} too small"
            );
            assert!(
                k == MIN_K
                    || get_keccak_capacity(usable_rows(k - 1), KECCAK_ROWS_PER_ROUND)
                        < permutations,
                "keccak {size}: k={k} larger than needed"
            );
        }
    }
}

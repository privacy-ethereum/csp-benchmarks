use std::collections::{BTreeMap, BTreeSet};

use flock_prover::chain::{ChainShiftProof, verify_chain_shift};
use flock_prover::challenger::{Challenger, FsChallenger};
use flock_prover::field::F128;
use flock_prover::pcs::{
    self, Commitment, DirectEqInd, LOG_PACKING, PackedDirectClaim, PackedDirectClaimRef, PcsParams,
};
use flock_prover::r1cs::BlockR1cs;
use flock_prover::r1cs_hashes::chain_common::{ChainFold, ChainLayout};
use flock_prover::r1cs_hashes::sha2::{self, Compression, Sha256HybridSetup};
use flock_prover::zerocheck::PaddingSpec;
use serde::{Deserialize, Serialize};

use crate::{SHA256_BLOCK_BYTES, SHA256_LEN_BYTES, sha256_compression_count};

#[derive(Debug)]
pub struct FullSha256Setup {
    pub r1cs: BlockR1cs,
    pub pcs_params: PcsParams,
    input_size: usize,
    pub n_compressions: usize,
    core: Sha256HybridSetup,
    compressions: Vec<Compression>,
    digest_words: [u32; 8],
    chain_end_words: [u32; 8],
    required_cells: BTreeSet<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullSha256Proof {
    pub zerocheck: flock_prover::zerocheck::ZerocheckProof,
    pub lincheck: flock_prover::lincheck::LincheckProof,
    pub shift: ChainShiftProof,
    pub pcs_open: pcs::BatchOpeningProof,
    pub commitment: Commitment,
    pub opened_cells: Vec<CellOpening>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CellOpening {
    packed_index: usize,
    value: F128,
}

#[allow(dead_code)]
#[derive(Debug)]
enum FullSha256VerifyError {
    R1cs(flock_prover::verifier::VerifyError),
    Chain(flock_prover::chain::ChainError),
    Pcs(pcs::VerifyError),
    OpenedCells,
    Relation,
}

pub fn prepare(input_size: usize) -> FullSha256Setup {
    let _ = flock_prover::init_perf_thread_pool();

    let (mut compressions, digest_words) = build_message_compressions(input_size);
    let n_compressions = compressions.len();
    let n_slots = n_compressions.max(8).next_power_of_two();
    let mut chain_cv = digest_words;
    while compressions.len() < n_slots {
        let filler = [0u32; 16];
        compressions.push((chain_cv, filler));
        chain_cv = sha2::sha256_compress(&chain_cv, &filler);
    }

    let core = Sha256HybridSetup::new(n_slots);
    let r1cs = core.r1cs.clone();
    let pcs_params = core.pcs_params.clone();
    let required_cells = required_cells(input_size, n_compressions);

    FullSha256Setup {
        r1cs,
        pcs_params,
        input_size,
        n_compressions,
        core,
        compressions,
        digest_words,
        chain_end_words: chain_cv,
        required_cells,
    }
}

pub fn prove(setup: &FullSha256Setup, transcript_domain: &[u8]) -> FullSha256Proof {
    let (z_packed, a_packed, b_packed, z_lincheck) =
        sha2::generate_witness_with_ab_packed_and_lincheck(
            &setup.compressions,
            setup.core.n_blocks_log(),
        );

    let mut challenger = FsChallenger::new(transcript_domain);
    let core = flock_prover::prover::prove_fast_core(
        &setup.r1cs,
        &setup.pcs_params,
        z_packed,
        a_packed,
        b_packed,
        z_lincheck,
        setup.r1cs.csc_lincheck_circuit(),
        &mut challenger,
    );

    let tau_pos = challenger.sample_f128_vec(sha2::CHAIN_LAYOUT.tau_pos_len());
    let fold = ChainFold::new(&sha2::CHAIN_LAYOUT, tau_pos);
    let (in_vals, out_vals) = flock_prover::r1cs_hashes::chain_common::fold_in_out(
        &sha2::CHAIN_LAYOUT,
        &core.z_packed,
        &fold,
    );
    let (shift, chain_claims) =
        flock_prover::chain::prove_chain_shift(&in_vals, &out_vals, &mut challenger);
    let chain_claim = flock_prover::r1cs_hashes::chain_common::assemble_chain_claim(
        &sha2::CHAIN_LAYOUT,
        &fold,
        &chain_claims,
    );

    let opened_cells = collect_opened_cells(&core.z_packed, &setup.required_cells);
    let direct_claims: Vec<PackedDirectClaim> = opened_cells
        .iter()
        .map(|cell| packed_direct_claim(setup.r1cs.m, *cell))
        .collect();
    let mut packed_direct = Vec::with_capacity(1 + direct_claims.len());
    packed_direct.push(chain_claim);
    packed_direct.extend(direct_claims);

    let padding = PaddingSpec {
        k_log: setup.r1cs.k_log,
        useful_bits_per_block: setup.r1cs.useful_bits,
    };
    let ab_x_outer = quirky_x_outer_full(&core.ab.point);
    let c_x_outer = quirky_x_outer_full(&core.c.point);
    let pre_ab = core.s_hat_v_ab.as_deref();
    let pre_c = Some(core.s_hat_v_c.as_slice());
    let pcs_open = pcs::open_batch_mixed_with_precomputed_s_hat_v(
        &core.z_packed,
        &core.prover_data,
        &core.commitment,
        &[ab_x_outer.as_slice(), c_x_outer.as_slice()],
        &[pre_ab, pre_c],
        &packed_direct,
        &padding,
        &mut challenger,
    );

    FullSha256Proof {
        zerocheck: core.zc_proof,
        lincheck: core.lc_proof,
        shift,
        pcs_open,
        commitment: core.commitment,
        opened_cells,
    }
}

pub fn verify(setup: &FullSha256Setup, proof: &FullSha256Proof, transcript_domain: &[u8]) {
    verify_result(setup, proof, transcript_domain)
        .expect("Flock full SHA-256 digest proof must verify");
}

fn verify_result(
    setup: &FullSha256Setup,
    proof: &FullSha256Proof,
    transcript_domain: &[u8],
) -> Result<(), FullSha256VerifyError> {
    let opened = opened_cell_map(&proof.opened_cells, &setup.required_cells)?;
    check_relations(setup, &opened)?;

    let mut challenger = FsChallenger::new(transcript_domain);
    let (ab, c) = flock_prover::verifier::verify_core(
        &setup.r1cs,
        &proof.zerocheck,
        &proof.lincheck,
        &proof.commitment,
        setup.r1cs.csc_lincheck_circuit(),
        &mut challenger,
    )
    .map_err(FullSha256VerifyError::R1cs)?;

    let tau_pos = challenger.sample_f128_vec(sha2::CHAIN_LAYOUT.tau_pos_len());
    let fold = ChainFold::new(&sha2::CHAIN_LAYOUT, tau_pos);
    let x0_r = fold.fold_public_phys(&sha2::cv_to_phys_bits(&sha2::SHA256_IV));
    let xlast_r = fold.fold_public_phys(&sha2::cv_to_phys_bits(&setup.chain_end_words));
    let chain_claims = verify_chain_shift(
        &proof.shift,
        x0_r,
        xlast_r,
        setup.core.n_blocks_log(),
        &mut challenger,
    )
    .map_err(FullSha256VerifyError::Chain)?;
    let chain_point = build_chain_claim_point(&sha2::CHAIN_LAYOUT, &fold, &chain_claims);

    let ab_x_outer = quirky_x_outer_full(&ab.point);
    let c_x_outer = quirky_x_outer_full(&c.point);
    let opened_points: Vec<Vec<F128>> = proof
        .opened_cells
        .iter()
        .map(|cell| packed_index_point(setup.r1cs.m, cell.packed_index))
        .collect();
    let mut direct_refs = Vec::with_capacity(1 + proof.opened_cells.len());
    direct_refs.push(PackedDirectClaimRef {
        point: chain_point.as_slice(),
        value: chain_claims.value,
    });
    direct_refs.extend(
        proof
            .opened_cells
            .iter()
            .zip(opened_points.iter())
            .map(|(cell, point)| PackedDirectClaimRef {
                point: point.as_slice(),
                value: cell.value,
            }),
    );

    pcs::verify_opening_batch_mixed(
        &proof.commitment,
        &[ab.value, c.value],
        &[ab.point.z_skip, c.point.z_skip],
        &[ab_x_outer.as_slice(), c_x_outer.as_slice()],
        &direct_refs,
        &proof.pcs_open,
        &mut challenger,
    )
    .map_err(FullSha256VerifyError::Pcs)?;

    Ok(())
}

pub fn num_constraints(setup: &FullSha256Setup) -> usize {
    setup.r1cs.useful_bits * setup.r1cs.n_outer()
}

pub fn proof_size(proof: &FullSha256Proof) -> usize {
    bincode::serialize(proof)
        .expect("Flock full SHA-256 proof must serialize")
        .len()
}

fn build_message_compressions(input_size: usize) -> (Vec<Compression>, [u32; 8]) {
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

    let mut cv = sha2::SHA256_IV;
    let mut blocks = Vec::with_capacity(padded.len() / SHA256_BLOCK_BYTES);
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
        cv = sha2::sha256_compress(&cv, &message_block);
    }

    let digest: Vec<u8> = cv.iter().flat_map(|word| word.to_be_bytes()).collect();
    assert_eq!(digest, expected_digest);
    (blocks, cv)
}

fn required_cells(input_size: usize, n_compressions: usize) -> BTreeSet<usize> {
    let mut cells = BTreeSet::new();
    let final_slot = n_compressions - 1;

    for cell in 0..2 {
        cells.insert(h_out_cell(final_slot, cell));
    }

    let total_bytes = n_compressions * SHA256_BLOCK_BYTES;
    for byte_idx in input_size..total_bytes {
        let block_idx = byte_idx / SHA256_BLOCK_BYTES;
        let byte_in_block = byte_idx % SHA256_BLOCK_BYTES;
        let word = byte_in_block / 4;
        let byte_in_word = byte_in_block % 4;
        for bit in 0..8 {
            let word_bit = (3 - byte_in_word) * 8 + bit;
            cells.insert(cell_for_bit(global_bit(
                block_idx,
                sha2::m_bit(word, word_bit),
            )));
        }
    }

    cells
}

fn check_relations(
    setup: &FullSha256Setup,
    opened: &BTreeMap<usize, F128>,
) -> Result<(), FullSha256VerifyError> {
    let final_slot = setup.n_compressions - 1;
    for cell in 0..2 {
        let value = opened_value(opened, h_out_cell(final_slot, cell))?;
        if value != digest_cell_value(&setup.digest_words, cell * 4) {
            return Err(FullSha256VerifyError::Relation);
        }
    }

    let n_compressions = setup.n_compressions;
    let total_bytes = n_compressions * SHA256_BLOCK_BYTES;
    let bit_len = (setup.input_size as u64) * 8;
    let expected_len_bytes = bit_len.to_be_bytes();
    for byte_idx in setup.input_size..total_bytes {
        let expected = if byte_idx == setup.input_size {
            0x80
        } else if byte_idx >= total_bytes - SHA256_LEN_BYTES {
            expected_len_bytes[byte_idx - (total_bytes - SHA256_LEN_BYTES)]
        } else {
            0
        };
        check_padding_byte(opened, byte_idx, expected)?;
    }

    Ok(())
}

fn check_padding_byte(
    opened: &BTreeMap<usize, F128>,
    byte_idx: usize,
    expected: u8,
) -> Result<(), FullSha256VerifyError> {
    let block_idx = byte_idx / SHA256_BLOCK_BYTES;
    let byte_in_block = byte_idx % SHA256_BLOCK_BYTES;
    let word = byte_in_block / 4;
    let byte_in_word = byte_in_block % 4;
    for bit in 0..8 {
        let word_bit = (3 - byte_in_word) * 8 + bit;
        let bit_index = global_bit(block_idx, sha2::m_bit(word, word_bit));
        let value = opened_value(opened, cell_for_bit(bit_index))?;
        let got = f128_bit(value, bit_index & 127);
        let want = (expected >> bit) & 1 == 1;
        if got != want {
            return Err(FullSha256VerifyError::Relation);
        }
    }
    Ok(())
}

fn collect_opened_cells(z_packed: &[F128], required: &BTreeSet<usize>) -> Vec<CellOpening> {
    required
        .iter()
        .map(|&packed_index| CellOpening {
            packed_index,
            value: z_packed[packed_index],
        })
        .collect()
}

fn opened_cell_map(
    opened_cells: &[CellOpening],
    required: &BTreeSet<usize>,
) -> Result<BTreeMap<usize, F128>, FullSha256VerifyError> {
    let mut map = BTreeMap::new();
    for cell in opened_cells {
        if map.insert(cell.packed_index, cell.value).is_some() {
            return Err(FullSha256VerifyError::OpenedCells);
        }
    }
    let got: BTreeSet<usize> = map.keys().copied().collect();
    if &got != required {
        return Err(FullSha256VerifyError::OpenedCells);
    }
    Ok(map)
}

fn opened_value(
    opened: &BTreeMap<usize, F128>,
    packed_index: usize,
) -> Result<F128, FullSha256VerifyError> {
    opened
        .get(&packed_index)
        .copied()
        .ok_or(FullSha256VerifyError::OpenedCells)
}

fn digest_cell_value(digest_words: &[u32; 8], first_word: usize) -> F128 {
    let mut value = F128::ZERO;
    for word_offset in 0..4 {
        let word = digest_words[first_word + word_offset];
        for bit in 0..32 {
            if (word >> bit) & 1 == 1 {
                let packed_bit = word_offset * 32 + bit;
                if packed_bit < 64 {
                    value.lo |= 1u64 << packed_bit;
                } else {
                    value.hi |= 1u64 << (packed_bit - 64);
                }
            }
        }
    }
    value
}

fn h_out_cell(slot: usize, cell: usize) -> usize {
    debug_assert!(cell < 2);
    cell_for_bit(global_bit(
        slot,
        sha2::H_OUT_BASE + cell * (1 << LOG_PACKING),
    ))
}

fn global_bit(slot: usize, local_bit: usize) -> usize {
    (slot << sha2::K_LOG) + local_bit
}

fn cell_for_bit(bit_index: usize) -> usize {
    bit_index >> LOG_PACKING
}

fn f128_bit(value: F128, bit: usize) -> bool {
    if bit < 64 {
        (value.lo >> bit) & 1 == 1
    } else {
        (value.hi >> (bit - 64)) & 1 == 1
    }
}

fn packed_direct_claim(m: usize, cell: CellOpening) -> PackedDirectClaim {
    let point = packed_index_point(m, cell.packed_index);
    let eq_ind = DirectEqInd::Sparse(pcs::ring_switch::build_eq_sparse(&point));
    PackedDirectClaim {
        point,
        value: cell.value,
        eq_ind,
    }
}

fn packed_index_point(m: usize, packed_index: usize) -> Vec<F128> {
    let point_len = m - LOG_PACKING;
    assert!(packed_index < (1usize << point_len));
    (0..point_len)
        .map(|bit| {
            if (packed_index >> bit) & 1 == 1 {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect()
}

fn build_chain_claim_point(
    layout: &ChainLayout,
    fold: &ChainFold,
    claims: &flock_prover::chain::ChainClaims,
) -> Vec<F128> {
    let high = layout.high_zeros();
    let mut point = Vec::with_capacity(fold.tau_pos.len() + 1 + high + claims.instance_point.len());
    point.extend_from_slice(&fold.tau_pos);
    point.push(claims.sel0);
    point.extend(std::iter::repeat_n(F128::ZERO, high));
    point.extend_from_slice(&claims.instance_point);
    point
}

fn quirky_x_outer_full(point: &flock_prover::lincheck::QuirkyPoint) -> Vec<F128> {
    let mut v = Vec::with_capacity(point.x_inner_rest.len() + point.x_outer.len());
    v.extend_from_slice(&point.x_inner_rest);
    v.extend_from_slice(&point.x_outer);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_digest_opening_rejects_mutation() {
        let setup = prepare(128);
        let mut proof = prove(&setup, b"full-sha256-digest-binding-test");
        let digest_cell = h_out_cell(setup.n_compressions - 1, 0);
        proof
            .opened_cells
            .iter_mut()
            .find(|cell| cell.packed_index == digest_cell)
            .expect("digest cell opening")
            .value
            .lo ^= 1;

        let result = verify_result(&setup, &proof, b"full-sha256-digest-binding-test");
        assert!(result.is_err());
    }
}

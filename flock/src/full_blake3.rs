use std::collections::{BTreeMap, BTreeSet};

use flock_prover::chain::{ChainShiftProof, verify_chain_shift};
use flock_prover::challenger::{Challenger, FsChallenger};
use flock_prover::field::F128;
use flock_prover::pcs::{
    self, Commitment, DirectEqInd, LOG_PACKING, PackedDirectClaim, PackedDirectClaimRef, PcsParams,
};
use flock_prover::r1cs::{BlockR1cs, WitnessLayout};
use flock_prover::r1cs_hashes::blake3::{self, Blake3Setup, Compression};
use flock_prover::r1cs_hashes::chain_common::{ChainFold, ChainLayout};
use serde::{Deserialize, Serialize};

use crate::blake3_compression_count;

const BLOCK_BYTES: usize = 64;
const CHUNK_BYTES: usize = 1024;
const CHUNK_START: u32 = 1;
const CHUNK_END: u32 = 2;
const PARENT: u32 = 4;
const ROOT: u32 = 8;

#[derive(Debug)]
pub struct FullBlake3Setup {
    pub r1cs: BlockR1cs,
    pub pcs_params: PcsParams,
    input_size: usize,
    pub n_compressions: usize,
    core: Blake3Setup,
    compressions: Vec<Compression>,
    expected_digest_words: [u32; 8],
    required_cells: BTreeSet<usize>,
    batch_major: bool,
    use_chain: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullBlake3Proof {
    pub proof: flock_prover::proof::R1csProof,
    pub chain_shift: Option<ChainShiftProof>,
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
enum FullBlake3VerifyError {
    R1cs(flock_prover::verifier::VerifyError),
    Pcs(pcs::VerifyError),
    OpenedCells,
    Relation,
}

pub fn prepare(input_size: usize, batch_major: bool) -> FullBlake3Setup {
    let use_chain = matches!(blake3_compression_count(input_size), 8 | 16);
    prepare_with_chain(input_size, batch_major, use_chain)
}

pub fn prepare_with_chain(
    input_size: usize,
    batch_major: bool,
    use_chain: bool,
) -> FullBlake3Setup {
    let _ = flock_prover::init_perf_thread_pool();

    let (compressions, expected_digest_words) = build_hash_compressions(input_size);
    let n_compressions = compressions.len();
    assert_eq!(n_compressions, blake3_compression_count(input_size));

    let core = if batch_major {
        Blake3Setup::new_batch_major(n_compressions)
    } else {
        Blake3Setup::new(n_compressions)
    };
    let r1cs = core.r1cs.clone();
    let pcs_params = core.pcs_params.clone();
    assert!(
        !use_chain || matches!(n_compressions, 8 | 16),
        "chunk-chain proof requires exactly 8 or 16 compressions"
    );
    let required_cells = required_cells(
        input_size,
        n_compressions,
        core.n_blocks_log(),
        batch_major,
        use_chain,
    );

    FullBlake3Setup {
        r1cs,
        pcs_params,
        input_size,
        n_compressions,
        core,
        compressions,
        expected_digest_words,
        required_cells,
        batch_major,
        use_chain,
    }
}

pub fn prove(setup: &FullBlake3Setup, transcript_domain: &[u8]) -> FullBlake3Proof {
    let (z_packed, a_packed, b_packed, z_lincheck) = if setup.batch_major {
        blake3::generate_witness_batch_major(&setup.compressions, setup.core.n_blocks_log())
    } else {
        blake3::generate_witness_with_ab_packed_and_lincheck(
            &setup.compressions,
            setup.core.n_blocks_log(),
        )
    };

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

    let (chain_shift, chain_claim) = if setup.use_chain {
        let tau_pos = challenger.sample_f128_vec(blake3::CHAIN_LAYOUT.tau_pos_len());
        let fold = ChainFold::new(&blake3::CHAIN_LAYOUT, tau_pos);
        let (in_vals, out_vals) = flock_prover::r1cs_hashes::chain_common::fold_in_out(
            &blake3::CHAIN_LAYOUT,
            setup.r1cs.layout,
            &core.z_packed,
            &fold,
        );
        let (shift, claims) =
            flock_prover::chain::prove_chain_shift(&in_vals, &out_vals, &mut challenger);
        let claim = flock_prover::r1cs_hashes::chain_common::assemble_chain_claim(
            &blake3::CHAIN_LAYOUT,
            setup.r1cs.layout,
            &fold,
            &claims,
        );
        (Some(shift), Some(claim))
    } else {
        (None, None)
    };

    let opened_cells = collect_opened_cells(&core.z_packed, &setup.required_cells);
    let direct_cell_claims: Vec<PackedDirectClaim> = opened_cells
        .iter()
        .map(|cell| packed_direct_claim(setup.r1cs.m, *cell))
        .collect();
    let mut direct_claims =
        Vec::with_capacity(usize::from(chain_claim.is_some()) + opened_cells.len());
    direct_claims.extend(chain_claim);
    direct_claims.extend(direct_cell_claims);

    let padding = setup.r1cs.padding_spec();
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
        &direct_claims,
        &padding,
        &mut challenger,
    );

    FullBlake3Proof {
        proof: flock_prover::proof::R1csProof {
            zerocheck: core.zc_proof,
            lincheck: core.lc_proof,
            pcs_open,
        },
        chain_shift,
        commitment: core.commitment,
        opened_cells,
    }
}

pub fn verify(setup: &FullBlake3Setup, proof: &FullBlake3Proof, transcript_domain: &[u8]) {
    verify_result(setup, proof, transcript_domain)
        .expect("Flock full BLAKE3 digest proof must verify");
}

fn verify_result(
    setup: &FullBlake3Setup,
    proof: &FullBlake3Proof,
    transcript_domain: &[u8],
) -> Result<(), FullBlake3VerifyError> {
    let opened = opened_cell_map(&proof.opened_cells, &setup.required_cells)?;
    check_relations(setup, &opened)?;

    let mut challenger = FsChallenger::new(transcript_domain);
    let (ab, c) = flock_prover::verifier::verify_core(
        &setup.r1cs,
        &proof.proof.zerocheck,
        &proof.proof.lincheck,
        &proof.commitment,
        setup.r1cs.csc_lincheck_circuit(),
        &mut challenger,
    )
    .map_err(FullBlake3VerifyError::R1cs)?;

    let chain_claim = if setup.use_chain {
        let shift = proof
            .chain_shift
            .as_ref()
            .ok_or(FullBlake3VerifyError::Relation)?;
        let tau_pos = challenger.sample_f128_vec(blake3::CHAIN_LAYOUT.tau_pos_len());
        let fold = ChainFold::new(&blake3::CHAIN_LAYOUT, tau_pos);
        let x0_r = fold.fold_public_phys(&blake3::cv_to_phys_bits(&blake3::BLAKE3_IV));
        let xlast_r = fold.fold_public_phys(&blake3::cv_to_phys_bits(&setup.expected_digest_words));
        let claims = verify_chain_shift(
            shift,
            x0_r,
            xlast_r,
            setup.core.n_blocks_log(),
            &mut challenger,
        )
        .map_err(|_| FullBlake3VerifyError::Relation)?;
        Some((
            build_chain_claim_point(&blake3::CHAIN_LAYOUT, setup.r1cs.layout, &fold, &claims),
            claims.value,
        ))
    } else if proof.chain_shift.is_some() {
        return Err(FullBlake3VerifyError::Relation);
    } else {
        None
    };

    let ab_x_outer = quirky_x_outer_full(&ab.point);
    let c_x_outer = quirky_x_outer_full(&c.point);
    let opened_points: Vec<Vec<F128>> = proof
        .opened_cells
        .iter()
        .map(|cell| packed_index_point(setup.r1cs.m, cell.packed_index))
        .collect();
    let mut direct_refs =
        Vec::with_capacity(usize::from(chain_claim.is_some()) + opened_points.len());
    if let Some((point, value)) = chain_claim.as_ref() {
        direct_refs.push(PackedDirectClaimRef {
            point: point.as_slice(),
            value: *value,
        });
    }
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
        &proof.proof.pcs_open,
        &mut challenger,
    )
    .map_err(FullBlake3VerifyError::Pcs)?;

    Ok(())
}

pub fn num_constraints(setup: &FullBlake3Setup) -> usize {
    setup.r1cs.useful_bits * setup.r1cs.n_outer()
}

pub fn proof_size(proof: &FullBlake3Proof) -> usize {
    bincode::serialize(proof)
        .expect("Flock full BLAKE3 proof must serialize")
        .len()
}

fn build_hash_compressions(input_size: usize) -> (Vec<Compression>, [u32; 8]) {
    assert!(
        matches!(input_size, 128 | 256 | 512 | 1024 | 2048),
        "unsupported BLAKE3 benchmark size"
    );

    let (message, expected_digest) = utils::generate_blake3_input(input_size);
    let message_blocks: Vec<[u32; 16]> = message
        .chunks_exact(BLOCK_BYTES)
        .map(|block| {
            std::array::from_fn(|word| {
                let start = word * 4;
                u32::from_le_bytes(block[start..start + 4].try_into().expect("BLAKE3 word"))
            })
        })
        .collect();

    let mut compressions = Vec::with_capacity(blake3_compression_count(input_size));
    let digest_words = if input_size <= CHUNK_BYTES {
        build_chunk(&message_blocks, 0, true, &mut compressions)
    } else {
        let child_0 = build_chunk(&message_blocks[..16], 0, false, &mut compressions);
        let child_1 = build_chunk(&message_blocks[16..], 1, false, &mut compressions);
        let parent_block = std::array::from_fn(|word| {
            if word < 8 {
                child_0[word]
            } else {
                child_1[word - 8]
            }
        });
        let parent = (
            blake3::BLAKE3_IV,
            parent_block,
            0,
            BLOCK_BYTES as u32,
            PARENT | ROOT,
        );
        compressions.push(parent);
        low_cv(blake3::blake3_compress(
            &parent.0, &parent.1, parent.2, parent.3, parent.4,
        ))
    };

    let digest: Vec<u8> = digest_words
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect();
    assert_eq!(digest, expected_digest, "BLAKE3 compression tree mismatch");

    (compressions, digest_words)
}

fn build_chunk(
    blocks: &[[u32; 16]],
    chunk_counter: u64,
    is_root: bool,
    compressions: &mut Vec<Compression>,
) -> [u32; 8] {
    let mut cv = blake3::BLAKE3_IV;
    for (index, block) in blocks.iter().enumerate() {
        let is_last = index + 1 == blocks.len();
        let flags = if index == 0 { CHUNK_START } else { 0 }
            | if is_last { CHUNK_END } else { 0 }
            | if is_last && is_root { ROOT } else { 0 };
        let compression = (cv, *block, chunk_counter, BLOCK_BYTES as u32, flags);
        compressions.push(compression);
        cv = low_cv(blake3::blake3_compress(
            &compression.0,
            &compression.1,
            compression.2,
            compression.3,
            compression.4,
        ));
    }
    cv
}

fn low_cv(state: [u32; 16]) -> [u32; 8] {
    state[..8].try_into().expect("eight-word chaining value")
}

fn required_cells(
    input_size: usize,
    n_compressions: usize,
    n_blocks_log: usize,
    batch_major: bool,
    use_chain: bool,
) -> BTreeSet<usize> {
    let mut cells = BTreeSet::new();

    for slot in 0..n_compressions {
        if !use_chain {
            insert_semantic_range(
                &mut cells,
                slot,
                blake3::CV_BASE,
                8 * 32,
                n_blocks_log,
                batch_major,
            );
            insert_semantic_range(
                &mut cells,
                slot,
                blake3::OUT_LO_BASE,
                8 * 32,
                n_blocks_log,
                batch_major,
            );
        }
        insert_semantic_range(
            &mut cells,
            slot,
            blake3::T_LO_BASE,
            4 * 32,
            n_blocks_log,
            batch_major,
        );
    }

    if input_size == 2048 {
        insert_semantic_range(
            &mut cells,
            n_compressions - 1,
            blake3::M_BASE,
            16 * 32,
            n_blocks_log,
            batch_major,
        );
    }

    cells
}

fn insert_semantic_range(
    cells: &mut BTreeSet<usize>,
    slot: usize,
    local_first_bit: usize,
    bit_len: usize,
    n_blocks_log: usize,
    batch_major: bool,
) {
    for local_bit in local_first_bit..local_first_bit + bit_len {
        let physical_bit = witness_bit(slot, local_bit, n_blocks_log, batch_major);
        cells.insert(cell_for_bit(physical_bit));
    }
}

fn check_relations(
    setup: &FullBlake3Setup,
    opened: &BTreeMap<usize, F128>,
) -> Result<(), FullBlake3VerifyError> {
    for (slot, compression) in setup.compressions.iter().enumerate() {
        check_word(setup, opened, slot, blake3::T_LO_BASE, compression.2 as u32)?;
        check_word(
            setup,
            opened,
            slot,
            blake3::T_HI_BASE,
            (compression.2 >> 32) as u32,
        )?;
        check_word(setup, opened, slot, blake3::BLEN_BASE, compression.3)?;
        check_word(setup, opened, slot, blake3::FLAGS_BASE, compression.4)?;
    }

    if setup.use_chain {
        return Ok(());
    }

    if setup.input_size <= CHUNK_BYTES {
        check_cv(setup, opened, 0, blake3::CV_BASE, &blake3::BLAKE3_IV)?;
        for slot in 1..setup.n_compressions {
            check_equal_semantic_ranges(
                setup,
                opened,
                slot,
                blake3::CV_BASE,
                slot - 1,
                blake3::OUT_LO_BASE,
                8 * 32,
            )?;
        }
    } else {
        debug_assert_eq!(setup.n_compressions, 33);
        for slot in [0, 16, 32] {
            check_cv(setup, opened, slot, blake3::CV_BASE, &blake3::BLAKE3_IV)?;
        }
        for slot in 1..16 {
            check_equal_semantic_ranges(
                setup,
                opened,
                slot,
                blake3::CV_BASE,
                slot - 1,
                blake3::OUT_LO_BASE,
                8 * 32,
            )?;
        }
        for slot in 17..32 {
            check_equal_semantic_ranges(
                setup,
                opened,
                slot,
                blake3::CV_BASE,
                slot - 1,
                blake3::OUT_LO_BASE,
                8 * 32,
            )?;
        }
        check_equal_semantic_ranges(
            setup,
            opened,
            32,
            blake3::M_BASE,
            15,
            blake3::OUT_LO_BASE,
            8 * 32,
        )?;
        check_equal_semantic_ranges(
            setup,
            opened,
            32,
            blake3::M_BASE + 8 * 32,
            31,
            blake3::OUT_LO_BASE,
            8 * 32,
        )?;
    }

    let final_slot = setup.n_compressions - 1;
    check_cv(
        setup,
        opened,
        final_slot,
        blake3::OUT_LO_BASE,
        &setup.expected_digest_words,
    )
}

fn check_cv(
    setup: &FullBlake3Setup,
    opened: &BTreeMap<usize, F128>,
    slot: usize,
    base: usize,
    expected: &[u32; 8],
) -> Result<(), FullBlake3VerifyError> {
    for (word, &value) in expected.iter().enumerate() {
        check_word(setup, opened, slot, base + word * 32, value)?;
    }
    Ok(())
}

fn check_word(
    setup: &FullBlake3Setup,
    opened: &BTreeMap<usize, F128>,
    slot: usize,
    local_first_bit: usize,
    expected: u32,
) -> Result<(), FullBlake3VerifyError> {
    for bit in 0..32 {
        let got = opened_bit(opened, setup_bit(setup, slot, local_first_bit + bit))?;
        let want = (expected >> bit) & 1 == 1;
        if got != want {
            return Err(FullBlake3VerifyError::Relation);
        }
    }
    Ok(())
}

fn check_equal_semantic_ranges(
    setup: &FullBlake3Setup,
    opened: &BTreeMap<usize, F128>,
    lhs_slot: usize,
    lhs_local_first: usize,
    rhs_slot: usize,
    rhs_local_first: usize,
    bit_len: usize,
) -> Result<(), FullBlake3VerifyError> {
    for bit in 0..bit_len {
        let lhs = setup_bit(setup, lhs_slot, lhs_local_first + bit);
        let rhs = setup_bit(setup, rhs_slot, rhs_local_first + bit);
        if opened_bit(opened, lhs)? != opened_bit(opened, rhs)? {
            return Err(FullBlake3VerifyError::Relation);
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
) -> Result<BTreeMap<usize, F128>, FullBlake3VerifyError> {
    let mut map = BTreeMap::new();
    for cell in opened_cells {
        if map.insert(cell.packed_index, cell.value).is_some() {
            return Err(FullBlake3VerifyError::OpenedCells);
        }
    }
    let got: BTreeSet<usize> = map.keys().copied().collect();
    if &got != required {
        return Err(FullBlake3VerifyError::OpenedCells);
    }
    Ok(map)
}

fn opened_bit(
    opened: &BTreeMap<usize, F128>,
    bit_index: usize,
) -> Result<bool, FullBlake3VerifyError> {
    let value = opened
        .get(&cell_for_bit(bit_index))
        .copied()
        .ok_or(FullBlake3VerifyError::OpenedCells)?;
    Ok(f128_bit(value, bit_index & 127))
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

fn setup_bit(setup: &FullBlake3Setup, slot: usize, local_bit: usize) -> usize {
    witness_bit(
        slot,
        local_bit,
        setup.core.n_blocks_log(),
        setup.batch_major,
    )
}

fn witness_bit(slot: usize, local_bit: usize, n_blocks_log: usize, batch_major: bool) -> usize {
    if batch_major {
        (local_bit & 127) | (slot << 7) | ((local_bit >> 7) << (7 + n_blocks_log))
    } else {
        (slot << blake3::K_LOG) | local_bit
    }
}

fn cell_for_bit(bit_index: usize) -> usize {
    bit_index >> LOG_PACKING
}

fn build_chain_claim_point(
    layout: &ChainLayout,
    witness_layout: WitnessLayout,
    fold: &ChainFold,
    claims: &flock_prover::chain::ChainClaims,
) -> Vec<F128> {
    let high = layout.high_zeros();
    let mut point = Vec::with_capacity(fold.tau_pos.len() + 1 + high + claims.instance_point.len());
    if witness_layout == WitnessLayout::BatchMajor {
        point.extend_from_slice(&claims.instance_point);
    }
    point.extend_from_slice(&fold.tau_pos);
    point.push(claims.sel0);
    point.extend(std::iter::repeat_n(F128::ZERO, high));
    if witness_layout == WitnessLayout::RowMajor {
        point.extend_from_slice(&claims.instance_point);
    }
    point
}

fn quirky_x_outer_full(point: &flock_prover::lincheck::QuirkyPoint) -> Vec<F128> {
    let mut value = Vec::with_capacity(point.x_inner_rest.len() + point.x_outer.len());
    value.extend_from_slice(&point.x_inner_rest);
    value.extend_from_slice(&point.x_outer);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flip_opened_bit(proof: &mut FullBlake3Proof, physical_bit: usize) {
        let opening = proof
            .opened_cells
            .iter_mut()
            .find(|cell| cell.packed_index == cell_for_bit(physical_bit))
            .expect("target cell opening");
        let bit = physical_bit & 127;
        if bit < 64 {
            opening.value.lo ^= 1u64 << bit;
        } else {
            opening.value.hi ^= 1u64 << (bit - 64);
        }
    }

    fn witness_word(z_packed: &[F128], setup: &FullBlake3Setup, slot: usize, base: usize) -> u32 {
        (0..32).fold(0, |word, bit| {
            let physical = setup_bit(setup, slot, base + bit);
            word | (u32::from(f128_bit(z_packed[cell_for_bit(physical)], physical & 127)) << bit)
        })
    }

    #[test]
    fn complete_compression_tree_matches_reference() {
        for input_size in [128, 256, 512, 1024, 2048] {
            let (compressions, _) = build_hash_compressions(input_size);
            assert_eq!(compressions.len(), blake3_compression_count(input_size));
        }
    }

    #[test]
    fn batch_major_semantic_inputs_match_compressions() {
        let setup = prepare(128, true);
        let (z_packed, _, _, _) =
            blake3::generate_witness_batch_major(&setup.compressions, setup.core.n_blocks_log());

        for (slot, compression) in setup.compressions.iter().enumerate() {
            for (word, &expected) in compression.0.iter().enumerate() {
                assert_eq!(
                    witness_word(&z_packed, &setup, slot, blake3::CV_BASE + word * 32),
                    expected
                );
            }
            for (word, &expected) in compression.1.iter().enumerate() {
                assert_eq!(
                    witness_word(&z_packed, &setup, slot, blake3::M_BASE + word * 32),
                    expected
                );
            }
            assert_eq!(
                witness_word(&z_packed, &setup, slot, blake3::T_LO_BASE),
                compression.2 as u32
            );
            assert_eq!(
                witness_word(&z_packed, &setup, slot, blake3::T_HI_BASE),
                (compression.2 >> 32) as u32
            );
            assert_eq!(
                witness_word(&z_packed, &setup, slot, blake3::BLEN_BASE),
                compression.3
            );
            assert_eq!(
                witness_word(&z_packed, &setup, slot, blake3::FLAGS_BASE),
                compression.4
            );
            let output = blake3::blake3_compress(
                &compression.0,
                &compression.1,
                compression.2,
                compression.3,
                compression.4,
            );
            for (word, &expected) in output[..8].iter().enumerate() {
                assert_eq!(
                    witness_word(&z_packed, &setup, slot, blake3::OUT_LO_BASE + word * 32),
                    expected
                );
            }
        }
    }

    #[test]
    fn bound_openings_reject_mutations() {
        let setup = prepare(128, true);
        let proof = prove(&setup, b"full-blake3-opening-binding-test");
        let targets = [
            ("counter", setup_bit(&setup, 0, blake3::T_LO_BASE)),
            ("flags", setup_bit(&setup, 0, blake3::FLAGS_BASE)),
            ("chaining value", setup_bit(&setup, 1, blake3::CV_BASE)),
            (
                "digest",
                setup_bit(&setup, setup.n_compressions - 1, blake3::OUT_LO_BASE),
            ),
        ];

        for (label, physical_bit) in targets {
            let mut mutated = proof.clone();
            flip_opened_bit(&mut mutated, physical_bit);
            let result = verify_result(&setup, &mutated, b"full-blake3-opening-binding-test");
            assert!(
                matches!(result, Err(FullBlake3VerifyError::Relation)),
                "mutated {label} opening was not rejected by the public relation: {result:?}"
            );
        }
    }

    #[test]
    fn chunk_chain_roundtrip_and_tamper_rejection() {
        let setup = prepare(512, true);
        let proof = prove(&setup, b"full-blake3-chunk-chain-test");
        verify(&setup, &proof, b"full-blake3-chunk-chain-test");
        assert!(proof.chain_shift.is_some());

        let mut mutated = proof;
        mutated.chain_shift.as_mut().expect("chain proof").rounds[0]
            .0
            .lo ^= 1;
        assert!(verify_result(&setup, &mutated, b"full-blake3-chunk-chain-test").is_err());
    }

    #[test]
    #[ignore = "covers the largest BLAKE3 benchmark circuit"]
    fn parent_opening_rejects_mutation() {
        let setup = prepare(2048, true);
        let proof = prove(&setup, b"full-blake3-parent-binding-test");
        verify(&setup, &proof, b"full-blake3-parent-binding-test");

        let mut mutated = proof;
        flip_opened_bit(&mut mutated, setup_bit(&setup, 32, blake3::M_BASE));
        let result = verify_result(&setup, &mutated, b"full-blake3-parent-binding-test");
        assert!(
            matches!(result, Err(FullBlake3VerifyError::Relation)),
            "mutated parent opening was not rejected by the tree relation: {result:?}"
        );
    }
}

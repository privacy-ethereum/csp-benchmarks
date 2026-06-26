use std::collections::{BTreeMap, BTreeSet};

use flock_prover::challenger::FsChallenger;
use flock_prover::field::F128;
use flock_prover::pcs::{
    self, Commitment, DirectEqInd, LOG_PACKING, PackedDirectClaim, PackedDirectClaimRef, PcsParams,
};
use flock_prover::r1cs::BlockR1cs;
use flock_prover::r1cs_hashes::keccak::{self, KeccakSetup};
use flock_prover::zerocheck::PaddingSpec;
use serde::{Deserialize, Serialize};

use crate::{KECCAK256_RATE_BYTES, keccak256_permutation_count};

const KECCAK_STATE_BYTES: usize = 200;
const KECCAK256_DIGEST_BYTES: usize = 32;

#[derive(Debug)]
pub struct FullKeccakSetup {
    pub r1cs: BlockR1cs,
    pub pcs_params: PcsParams,
    input_size: usize,
    pub n_permutations: usize,
    core: KeccakSetup,
    initial_states: Vec<keccak::State>,
    expected_digest: [u8; KECCAK256_DIGEST_BYTES],
    required_cells: BTreeSet<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullKeccakProof {
    pub proof: flock_prover::proof::R1csProof,
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
enum FullKeccakVerifyError {
    R1cs(flock_prover::verifier::VerifyError),
    Pcs(pcs::VerifyError),
    OpenedCells,
    Relation,
}

pub fn prepare(input_size: usize) -> FullKeccakSetup {
    let _ = flock_prover::init_perf_thread_pool();

    let (initial_states, expected_digest) = build_sponge_states(input_size);
    let n_permutations = initial_states.len();
    let core = KeccakSetup::new(n_permutations);
    let r1cs = core.r1cs.clone();
    let pcs_params = core.pcs_params.clone();
    let required_cells = required_cells(input_size, n_permutations);

    FullKeccakSetup {
        r1cs,
        pcs_params,
        input_size,
        n_permutations,
        core,
        initial_states,
        expected_digest,
        required_cells,
    }
}

pub fn prove(setup: &FullKeccakSetup, transcript_domain: &[u8]) -> FullKeccakProof {
    let (z_packed, a_packed, b_packed, z_lincheck) =
        keccak::generate_witness_with_ab_packed_and_lincheck(
            &setup.initial_states,
            setup.core.n_keccaks_log(),
        );

    let mut challenger = FsChallenger::new(transcript_domain);
    let core = flock_prover::prover::prove_fast_core(
        &setup.r1cs,
        &setup.pcs_params,
        z_packed,
        a_packed,
        b_packed,
        z_lincheck,
        &keccak::KeccakLincheckCircuit,
        &mut challenger,
    );

    let opened_cells = collect_opened_cells(&core.z_packed, &setup.required_cells);
    let direct_claims: Vec<PackedDirectClaim> = opened_cells
        .iter()
        .map(|cell| packed_direct_claim(setup.r1cs.m, *cell))
        .collect();

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
        &direct_claims,
        &padding,
        &mut challenger,
    );

    FullKeccakProof {
        proof: flock_prover::proof::R1csProof {
            zerocheck: core.zc_proof,
            lincheck: core.lc_proof,
            pcs_open,
        },
        commitment: core.commitment,
        opened_cells,
    }
}

pub fn verify(setup: &FullKeccakSetup, proof: &FullKeccakProof, transcript_domain: &[u8]) {
    verify_result(setup, proof, transcript_domain)
        .expect("Flock full Keccak-256 digest proof must verify");
}

fn verify_result(
    setup: &FullKeccakSetup,
    proof: &FullKeccakProof,
    transcript_domain: &[u8],
) -> Result<(), FullKeccakVerifyError> {
    let opened = opened_cell_map(&proof.opened_cells, &setup.required_cells)?;
    check_relations(setup, &opened)?;

    let mut challenger = FsChallenger::new(transcript_domain);
    let (ab, c) = flock_prover::verifier::verify_core(
        &setup.r1cs,
        &proof.proof.zerocheck,
        &proof.proof.lincheck,
        &proof.commitment,
        &keccak::KeccakLincheckCircuit,
        &mut challenger,
    )
    .map_err(FullKeccakVerifyError::R1cs)?;

    let ab_x_outer = quirky_x_outer_full(&ab.point);
    let c_x_outer = quirky_x_outer_full(&c.point);
    let opened_points: Vec<Vec<F128>> = proof
        .opened_cells
        .iter()
        .map(|cell| packed_index_point(setup.r1cs.m, cell.packed_index))
        .collect();
    let direct_refs: Vec<PackedDirectClaimRef<'_>> = proof
        .opened_cells
        .iter()
        .zip(opened_points.iter())
        .map(|(cell, point)| PackedDirectClaimRef {
            point: point.as_slice(),
            value: cell.value,
        })
        .collect();

    pcs::verify_opening_batch_mixed(
        &proof.commitment,
        &[ab.value, c.value],
        &[ab.point.z_skip, c.point.z_skip],
        &[ab_x_outer.as_slice(), c_x_outer.as_slice()],
        &direct_refs,
        &proof.proof.pcs_open,
        &mut challenger,
    )
    .map_err(FullKeccakVerifyError::Pcs)?;

    Ok(())
}

pub fn num_constraints(setup: &FullKeccakSetup) -> usize {
    setup.r1cs.useful_bits * setup.r1cs.n_outer()
}

pub fn proof_size(proof: &FullKeccakProof) -> usize {
    bincode::serialize(proof)
        .expect("Flock full Keccak proof must serialize")
        .len()
}

fn build_sponge_states(input_size: usize) -> (Vec<keccak::State>, [u8; KECCAK256_DIGEST_BYTES]) {
    let (message, expected_digest) = utils::generate_keccak_input(input_size);
    let expected_digest: [u8; KECCAK256_DIGEST_BYTES] = expected_digest
        .try_into()
        .expect("Keccak-256 digest is 32 bytes");
    let blocks = padded_keccak_blocks(&message);
    assert_eq!(blocks.len(), keccak256_permutation_count(input_size));

    let mut state = [false; keccak::STATE_BITS];
    let mut initial_states = Vec::with_capacity(blocks.len());
    for block in &blocks {
        absorb_block(&mut state, block);
        initial_states.push(state);
        keccak::keccak_f(&mut state);
    }

    assert_eq!(squeeze_prefix(&state), expected_digest);
    (initial_states, expected_digest)
}

fn padded_keccak_blocks(message: &[u8]) -> Vec<[u8; KECCAK256_RATE_BYTES]> {
    let full_blocks = message.len() / KECCAK256_RATE_BYTES;
    let mut blocks = Vec::with_capacity(full_blocks + 1);

    for chunk in message.chunks_exact(KECCAK256_RATE_BYTES).take(full_blocks) {
        blocks.push(chunk.try_into().unwrap());
    }

    let remainder = &message[full_blocks * KECCAK256_RATE_BYTES..];
    let mut final_block = [0u8; KECCAK256_RATE_BYTES];
    final_block[..remainder.len()].copy_from_slice(remainder);
    final_block[remainder.len()] ^= 0x01;
    final_block[KECCAK256_RATE_BYTES - 1] ^= 0x80;
    blocks.push(final_block);
    blocks
}

fn absorb_block(state: &mut keccak::State, block: &[u8; KECCAK256_RATE_BYTES]) {
    for (byte_idx, byte) in block.iter().enumerate() {
        for bit_idx in 0..8 {
            if (byte >> bit_idx) & 1 == 1 {
                state[keccak_state_pos(byte_idx, bit_idx)] ^= true;
            }
        }
    }
}

fn squeeze_prefix(state: &keccak::State) -> [u8; KECCAK256_DIGEST_BYTES] {
    std::array::from_fn(|byte_idx| {
        let mut byte = 0u8;
        for bit_idx in 0..8 {
            if state[keccak_state_pos(byte_idx, bit_idx)] {
                byte |= 1 << bit_idx;
            }
        }
        byte
    })
}

fn required_cells(input_size: usize, n_permutations: usize) -> BTreeSet<usize> {
    let mut cells = BTreeSet::new();
    let final_slot = n_permutations - 1;

    for byte_idx in KECCAK256_RATE_BYTES..KECCAK_STATE_BYTES {
        for bit_idx in 0..8 {
            cells.insert(cell_for_bit(state0_global_bit(
                0,
                keccak_state_pos(byte_idx, bit_idx),
            )));
        }
    }

    for slot in 1..n_permutations {
        for byte_idx in KECCAK256_RATE_BYTES..KECCAK_STATE_BYTES {
            for bit_idx in 0..8 {
                let state_bit = keccak_state_pos(byte_idx, bit_idx);
                cells.insert(cell_for_bit(state0_global_bit(slot, state_bit)));
                cells.insert(cell_for_bit(state24_global_bit(slot - 1, state_bit)));
            }
        }
    }

    let final_remainder = input_size % KECCAK256_RATE_BYTES;
    for byte_idx in final_remainder..KECCAK256_RATE_BYTES {
        for bit_idx in 0..8 {
            let state_bit = keccak_state_pos(byte_idx, bit_idx);
            cells.insert(cell_for_bit(state0_global_bit(final_slot, state_bit)));
            if final_slot > 0 {
                cells.insert(cell_for_bit(state24_global_bit(final_slot - 1, state_bit)));
            }
        }
    }

    for byte_idx in 0..KECCAK256_DIGEST_BYTES {
        for bit_idx in 0..8 {
            cells.insert(cell_for_bit(state24_global_bit(
                final_slot,
                keccak_state_pos(byte_idx, bit_idx),
            )));
        }
    }

    cells
}

fn check_relations(
    setup: &FullKeccakSetup,
    opened: &BTreeMap<usize, F128>,
) -> Result<(), FullKeccakVerifyError> {
    for byte_idx in KECCAK256_RATE_BYTES..KECCAK_STATE_BYTES {
        for bit_idx in 0..8 {
            if opened_bit(
                opened,
                state0_global_bit(0, keccak_state_pos(byte_idx, bit_idx)),
            )? {
                return Err(FullKeccakVerifyError::Relation);
            }
        }
    }

    for slot in 1..setup.n_permutations {
        for byte_idx in KECCAK256_RATE_BYTES..KECCAK_STATE_BYTES {
            for bit_idx in 0..8 {
                let state_bit = keccak_state_pos(byte_idx, bit_idx);
                let lhs = opened_bit(opened, state0_global_bit(slot, state_bit))?;
                let rhs = opened_bit(opened, state24_global_bit(slot - 1, state_bit))?;
                if lhs != rhs {
                    return Err(FullKeccakVerifyError::Relation);
                }
            }
        }
    }

    let final_slot = setup.n_permutations - 1;
    let final_remainder = setup.input_size % KECCAK256_RATE_BYTES;
    for byte_idx in final_remainder..KECCAK256_RATE_BYTES {
        let fixed_byte = final_block_fixed_byte(final_remainder, byte_idx);
        for bit_idx in 0..8 {
            let state_bit = keccak_state_pos(byte_idx, bit_idx);
            let state0 = opened_bit(opened, state0_global_bit(final_slot, state_bit))?;
            let prev = if final_slot == 0 {
                false
            } else {
                opened_bit(opened, state24_global_bit(final_slot - 1, state_bit))?
            };
            let got = state0 ^ prev;
            let want = (fixed_byte >> bit_idx) & 1 == 1;
            if got != want {
                return Err(FullKeccakVerifyError::Relation);
            }
        }
    }

    for byte_idx in 0..KECCAK256_DIGEST_BYTES {
        let expected = setup.expected_digest[byte_idx];
        for bit_idx in 0..8 {
            let got = opened_bit(
                opened,
                state24_global_bit(final_slot, keccak_state_pos(byte_idx, bit_idx)),
            )?;
            let want = (expected >> bit_idx) & 1 == 1;
            if got != want {
                return Err(FullKeccakVerifyError::Relation);
            }
        }
    }

    Ok(())
}

fn final_block_fixed_byte(remainder: usize, byte_idx: usize) -> u8 {
    if byte_idx == remainder {
        0x01
    } else if byte_idx == KECCAK256_RATE_BYTES - 1 {
        0x80
    } else {
        0
    }
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
) -> Result<BTreeMap<usize, F128>, FullKeccakVerifyError> {
    let mut map = BTreeMap::new();
    for cell in opened_cells {
        if map.insert(cell.packed_index, cell.value).is_some() {
            return Err(FullKeccakVerifyError::OpenedCells);
        }
    }
    let got: BTreeSet<usize> = map.keys().copied().collect();
    if &got != required {
        return Err(FullKeccakVerifyError::OpenedCells);
    }
    Ok(map)
}

fn opened_bit(
    opened: &BTreeMap<usize, F128>,
    bit_index: usize,
) -> Result<bool, FullKeccakVerifyError> {
    let value = opened
        .get(&cell_for_bit(bit_index))
        .copied()
        .ok_or(FullKeccakVerifyError::OpenedCells)?;
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

fn state0_global_bit(slot: usize, state_bit: usize) -> usize {
    slot * keccak::K + keccak::z_pos_state(0, state_bit)
}

fn state24_global_bit(slot: usize, state_bit: usize) -> usize {
    slot * keccak::K + keccak::z_pos_state(24, state_bit)
}

fn cell_for_bit(bit_index: usize) -> usize {
    bit_index >> LOG_PACKING
}

fn keccak_state_pos(byte_idx: usize, bit_idx: usize) -> usize {
    let lane = byte_idx / 8;
    let lane_byte = byte_idx % 8;
    let x = lane % 5;
    let y = lane / 5;
    let z = lane_byte * 8 + bit_idx;
    keccak::state_idx(x, y, z)
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
        let mut proof = prove(&setup, b"full-keccak-digest-binding-test");
        let digest_cell = cell_for_bit(state24_global_bit(
            setup.n_permutations - 1,
            keccak_state_pos(0, 0),
        ));
        proof
            .opened_cells
            .iter_mut()
            .find(|cell| cell.packed_index == digest_cell)
            .expect("digest cell opening")
            .value
            .lo ^= 1;

        let result = verify_result(&setup, &proof, b"full-keccak-digest-binding-test");
        assert!(result.is_err());
    }
}

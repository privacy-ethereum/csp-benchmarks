#![no_std]

pub const HASH_PAIR_BYTES: usize = 64;
pub const HASH_DIGEST_BYTES: usize = 32;
pub const CONSTANT_OVERHEAD_INPUT_BYTES: usize = 8;
pub const CONSTANT_OVERHEAD_OUTPUT_BYTES: usize = 8;
pub const MERKLE_DEPTH: usize = 32;
pub const FAKE_MERKLE_BRANCH_COUNTS: [usize; 2] = [4, 32];
pub const HASH_COUNTS: [usize; 2] = [128, 2048];

const MAX_FAKE_AMOUNT: i64 = 65_535;

pub fn encode_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn encode_u64(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

pub fn evaluate_constant_overhead(input: &[u8]) -> [u8; CONSTANT_OVERHEAD_OUTPUT_BYTES] {
    assert_eq!(input.len(), CONSTANT_OVERHEAD_INPUT_BYTES);
    let x = read_u64_at(input, 0);
    let result = x.checked_add(2).expect("constant_overhead overflow");
    assert_eq!(result, 4);
    result.to_le_bytes()
}

pub fn evaluate_hashes<F>(input: &[u8], mut hash_pair: F) -> [u8; HASH_DIGEST_BYTES]
where
    F: FnMut(&[u8; HASH_PAIR_BYTES]) -> [u8; HASH_DIGEST_BYTES],
{
    let count = decode_count(input);
    assert_hash_count(count);

    let mut folded = [0u8; HASH_DIGEST_BYTES];
    for index in 0..count {
        let pair = hash_pair_input(index as u64);
        let digest = hash_pair(&pair);
        fold_digest(&mut folded, index, &digest);
    }
    folded
}

pub fn evaluate_real_merkle<F>(input: &[u8], mut hash_pair: F) -> [u8; HASH_DIGEST_BYTES]
where
    F: FnMut(&[u8; HASH_PAIR_BYTES]) -> [u8; HASH_DIGEST_BYTES],
{
    let branch_count = decode_count(input);
    assert_branch_count(branch_count);

    let mut folded = [0u8; HASH_DIGEST_BYTES];
    for branch in 0..branch_count {
        let mut acc = deterministic_digest(0x4c45_4146, branch as u64, 0);
        let path = path_index(branch as u64);
        for level in 0..MERKLE_DEPTH {
            let sibling = deterministic_digest(0x5349_424c, branch as u64, level as u64);
            let mut pair = [0u8; HASH_PAIR_BYTES];
            if ((path >> level) & 1) == 1 {
                pair[..HASH_DIGEST_BYTES].copy_from_slice(&sibling);
                pair[HASH_DIGEST_BYTES..].copy_from_slice(&acc);
            } else {
                pair[..HASH_DIGEST_BYTES].copy_from_slice(&acc);
                pair[HASH_DIGEST_BYTES..].copy_from_slice(&sibling);
            }
            acc = hash_pair(&pair);
        }
        fold_digest(&mut folded, branch, &acc);
    }
    folded
}

pub fn evaluate_fake_merkle(input: &[u8]) -> [u8; HASH_DIGEST_BYTES] {
    let branch_count = decode_count(input);
    evaluate_fake_merkle_count(branch_count)
}

pub fn evaluate_fake_merkle_count(branch_count: usize) -> [u8; HASH_DIGEST_BYTES] {
    assert_branch_count(branch_count);
    assert_eq!(branch_count % 4, 0);

    let mut folded = [0u8; HASH_DIGEST_BYTES];
    let mut group_inputs = 0i64;
    let mut group_outputs = 0i64;

    for branch in 0..branch_count {
        let amount = fake_amount(branch);
        assert!((0..=MAX_FAKE_AMOUNT).contains(&amount));
        let owner = fake_owner(branch);
        let leaf = checked_add_i64(owner, amount);
        let root = fake_merkle_root(branch, leaf);

        match branch % 4 {
            0 | 1 => group_inputs = checked_add_i64(group_inputs, amount),
            2 | 3 => group_outputs = checked_add_i64(group_outputs, amount),
            _ => unreachable!(),
        }
        if branch % 4 == 3 {
            assert_eq!(group_inputs, group_outputs);
            group_inputs = 0;
            group_outputs = 0;
        }

        fold_i64(&mut folded, branch, root);
    }

    folded
}

pub fn hash_pair_input(index: u64) -> [u8; HASH_PAIR_BYTES] {
    let mut pair = [0u8; HASH_PAIR_BYTES];
    let left = deterministic_digest(0x4841_5348, index, 0);
    let right = deterministic_digest(0x5041_4952, index, 1);
    pair[..HASH_DIGEST_BYTES].copy_from_slice(&left);
    pair[HASH_DIGEST_BYTES..].copy_from_slice(&right);
    pair
}

pub fn decode_count(input: &[u8]) -> usize {
    assert_eq!(input.len(), 4);
    read_u32_at(input, 0) as usize
}

pub fn assert_hash_count(count: usize) {
    assert!(HASH_COUNTS.contains(&count));
}

pub fn assert_branch_count(branch_count: usize) {
    assert!(FAKE_MERKLE_BRANCH_COUNTS.contains(&branch_count));
}

fn fake_merkle_root(branch: usize, leaf: i64) -> i64 {
    let mut acc = leaf;
    let path = path_index(branch as u64);
    for level in 0..MERKLE_DEPTH {
        let sibling = fake_sibling(branch, level);
        acc = if ((path >> level) & 1) == 1 {
            checked_add_i64(sibling, acc)
        } else {
            checked_add_i64(acc, sibling)
        };
    }
    acc
}

fn fake_amount(branch: usize) -> i64 {
    let group = (branch / 4) as i64;
    match branch % 4 {
        0 => 70 + group,
        1 => 50 + group,
        2 => 80 + group,
        3 => 40 + group,
        _ => unreachable!(),
    }
}

fn fake_owner(branch: usize) -> i64 {
    10_000 + branch as i64
}

fn fake_sibling(branch: usize, level: usize) -> i64 {
    1 + ((branch as i64 * 97 + level as i64 * 131) % 10_000)
}

fn checked_add_i64(lhs: i64, rhs: i64) -> i64 {
    lhs.checked_add(rhs).expect("fake Merkle addition overflow")
}

fn fold_i64(out: &mut [u8; HASH_DIGEST_BYTES], index: usize, value: i64) {
    let slot = (index % 4) * 8;
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&out[slot..slot + 8]);
    let current = i64::from_le_bytes(bytes);
    let folded = checked_add_i64(current, value);
    out[slot..slot + 8].copy_from_slice(&folded.to_le_bytes());
}

fn fold_digest(out: &mut [u8; HASH_DIGEST_BYTES], index: usize, digest: &[u8; HASH_DIGEST_BYTES]) {
    let tweak = (index as u8).wrapping_mul(17).wrapping_add(31);
    for (out_byte, digest_byte) in out.iter_mut().zip(digest.iter()) {
        *out_byte ^= digest_byte.wrapping_add(tweak);
    }
}

fn deterministic_digest(domain: u64, index: u64, offset: u64) -> [u8; HASH_DIGEST_BYTES] {
    let mut state = domain
        ^ index.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ offset.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let mut out = [0u8; HASH_DIGEST_BYTES];
    for chunk in out.chunks_exact_mut(8) {
        let word = splitmix64(&mut state);
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}

fn path_index(branch: u64) -> u64 {
    let mut state = 0x7061_7468_5f69_6478 ^ branch;
    splitmix64(&mut state)
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn read_u32_at(input: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&input[offset..offset + 4]);
    u32::from_le_bytes(bytes)
}

fn read_u64_at(input: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_hash(pair: &[u8; HASH_PAIR_BYTES]) -> [u8; HASH_DIGEST_BYTES] {
        let mut out = [0u8; HASH_DIGEST_BYTES];
        out.copy_from_slice(&pair[..HASH_DIGEST_BYTES]);
        out
    }

    #[test]
    fn hash_inputs_are_deterministic() {
        assert_eq!(hash_pair_input(7), hash_pair_input(7));
        assert_ne!(hash_pair_input(7), hash_pair_input(8));
    }

    #[test]
    fn fake_merkle_supports_requested_branch_counts() {
        assert_ne!(
            evaluate_fake_merkle_count(4),
            evaluate_fake_merkle_count(32)
        );
    }

    #[test]
    fn hash_count_evaluator_is_deterministic() {
        let input = encode_u32(128);
        assert_eq!(
            evaluate_hashes(&input, identity_hash),
            evaluate_hashes(&input, identity_hash)
        );
    }

    #[test]
    fn real_merkle_evaluator_is_deterministic() {
        let input = encode_u32(4);
        assert_eq!(
            evaluate_real_merkle(&input, identity_hash),
            evaluate_real_merkle(&input, identity_hash)
        );
    }
}

use sha2::{Digest, Sha256};
use sha3::Keccak256;
use targeted_guest::{
    CONSTANT_OVERHEAD_OUTPUT_BYTES, HASH_DIGEST_BYTES, encode_u32, encode_u64,
    evaluate_constant_overhead, evaluate_fake_merkle, evaluate_hashes, evaluate_real_merkle,
};

pub use targeted_guest::{FAKE_MERKLE_BRANCH_COUNTS, HASH_COUNTS, MERKLE_DEPTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteHashKind {
    Sha256,
    Keccak,
    Blake3,
}

pub fn generate_constant_overhead_input() -> (Vec<u8>, Vec<u8>) {
    let input = encode_u64(2).to_vec();
    let expected = evaluate_constant_overhead(&input).to_vec();
    (input, expected)
}

pub fn generate_hash_count_input(kind: ByteHashKind, hash_count: usize) -> (Vec<u8>, Vec<u8>) {
    let input = encode_u32(hash_count as u32).to_vec();
    let expected = evaluate_hash_count(kind, &input).to_vec();
    (input, expected)
}

pub fn generate_real_merkle_input(kind: ByteHashKind, branch_count: usize) -> (Vec<u8>, Vec<u8>) {
    let input = encode_u32(branch_count as u32).to_vec();
    let expected = evaluate_real_merkle_input(kind, &input).to_vec();
    (input, expected)
}

pub fn generate_fake_merkle_input(branch_count: usize) -> (Vec<u8>, Vec<u8>) {
    let input = encode_u32(branch_count as u32).to_vec();
    let expected = evaluate_fake_merkle(&input).to_vec();
    (input, expected)
}

pub fn evaluate_hash_count(kind: ByteHashKind, input: &[u8]) -> [u8; HASH_DIGEST_BYTES] {
    evaluate_hashes(input, |pair| hash_pair(kind, pair))
}

pub fn evaluate_real_merkle_input(kind: ByteHashKind, input: &[u8]) -> [u8; HASH_DIGEST_BYTES] {
    evaluate_real_merkle(input, |pair| hash_pair(kind, pair))
}

pub fn constant_overhead_expected_output() -> [u8; CONSTANT_OVERHEAD_OUTPUT_BYTES] {
    evaluate_constant_overhead(&encode_u64(2))
}

fn hash_pair(kind: ByteHashKind, pair: &[u8; 64]) -> [u8; HASH_DIGEST_BYTES] {
    match kind {
        ByteHashKind::Sha256 => Sha256::digest(pair).into(),
        ByteHashKind::Keccak => Keccak256::digest(pair).into(),
        ByteHashKind::Blake3 => *blake3::hash(pair).as_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_overhead_input_is_valid() {
        let (input, expected) = generate_constant_overhead_input();
        assert_eq!(expected, constant_overhead_expected_output());
        assert_eq!(evaluate_constant_overhead(&input).to_vec(), expected);
    }

    #[test]
    fn hash_count_generation_is_deterministic() {
        let first = generate_hash_count_input(ByteHashKind::Sha256, 128);
        let second = generate_hash_count_input(ByteHashKind::Sha256, 128);
        assert_eq!(first, second);
    }

    #[test]
    fn hash_kinds_have_distinct_outputs() {
        let (_, sha) = generate_hash_count_input(ByteHashKind::Sha256, 128);
        let (_, keccak) = generate_hash_count_input(ByteHashKind::Keccak, 128);
        let (_, blake3) = generate_hash_count_input(ByteHashKind::Blake3, 128);
        assert_ne!(sha, keccak);
        assert_ne!(sha, blake3);
        assert_ne!(keccak, blake3);
    }

    #[test]
    fn merkle_branch_counts_are_distinct() {
        let (_, four) = generate_fake_merkle_input(4);
        let (_, thirty_two) = generate_fake_merkle_input(32);
        assert_ne!(four, thirty_two);
    }

    #[test]
    fn real_merkle_generation_is_deterministic() {
        let first = generate_real_merkle_input(ByteHashKind::Keccak, 4);
        let second = generate_real_merkle_input(ByteHashKind::Keccak, 4);
        assert_eq!(first, second);
    }
}

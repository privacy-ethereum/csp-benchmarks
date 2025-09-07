#![cfg_attr(feature = "guest", no_std)]

use sha2::{Digest, Sha256};

#[jolt::provable]
fn sha2(_input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    // Ere is not handling program inputs for jolt at the moment.
    // For now, I'll hardcode a 2048 byte input.
    let input = [0xAA; 2048];
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

#![no_main]

use sha2::{Digest, Sha256};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input_bytes = sp1_zkvm::io::read_vec();

    let mut hasher = Sha256::new();
    hasher.update(input_bytes);
    let hash: [u8; 32] = hasher.finalize().into();

    sp1_zkvm::io::commit_slice(&hash);
}

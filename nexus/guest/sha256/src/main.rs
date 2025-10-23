#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;
use alloc::vec::Vec;
use nexus_rt::{read_private_input, write_public_output};
use sha2::{Digest, Sha256};

#[nexus_rt::main]
fn main() {
    // Read the serialized input
    let serialized_input: Vec<u8> = read_private_input().expect("failed to read input");

    // Deserialize to get the actual message bytes
    let message_bytes: Vec<u8> = postcard::from_bytes(&serialized_input)
        .expect("failed to deserialize input");

    // Compute SHA256
    let mut hasher = Sha256::new();
    hasher.update(&message_bytes);
    let hash: [u8; 32] = hasher.finalize().into();

    // Convert to Vec<u8> for output
    let hash_vec: Vec<u8> = hash.to_vec();
    write_public_output(&hash_vec).expect("failed to write output");
}

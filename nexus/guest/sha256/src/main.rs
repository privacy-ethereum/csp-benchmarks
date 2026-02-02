#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;

use nexus_rt::{read_private_input, write_public_output};
use sha2::{Digest, Sha256};

#[nexus_rt::main]
fn main() {
    let input: alloc::vec::Vec<u8> = read_private_input().expect("failed to read input");

    // Compute SHA256
    let mut hasher = Sha256::new();
    hasher.update(&input);
    let hash: [u8; 32] = hasher.finalize().into();

    // Write as Vec<u8> to match postcard decoding on host side
    write_public_output(&hash.to_vec()).expect("failed to write output");
}

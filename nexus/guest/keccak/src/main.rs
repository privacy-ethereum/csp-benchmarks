#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;

use nexus_rt::{
    keccak::{Hasher, Keccak},
    read_private_input, write_public_output,
};

#[nexus_rt::main]
fn main() {
    let input: alloc::vec::Vec<u8> = read_private_input().expect("failed to read input");

    let mut keccak = Keccak::v256();
    keccak.update(&input);
    let mut hash = [0u8; 32];
    keccak.finalize(&mut hash);

    write_public_output(&hash.to_vec()).expect("failed to write output");
}

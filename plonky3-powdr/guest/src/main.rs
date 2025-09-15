#![allow(unused_imports)]

use powdr_riscv_runtime;
use powdr_riscv_runtime::io::read;
use sha2::{Digest, Sha256};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let msg: Vec<u8> = read(1);

    let _ = sha2(&msg);
}

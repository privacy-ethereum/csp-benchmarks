use core::hint::black_box;

use openvm::io::{read_vec, reveal_bytes32};
use openvm_sha2::sha256;

fn main() {
    let input = read_vec();
    let hash = sha256(&black_box(input));
    reveal_bytes32(hash);
}

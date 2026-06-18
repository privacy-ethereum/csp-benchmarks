#![no_main]

use private_tx_guest::evaluate_private_tx;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input = sp1_zkvm::io::read_vec();
    let output = evaluate_private_tx(&input);
    sp1_zkvm::io::commit_slice(&output);
}

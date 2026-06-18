use openvm::io::{read_vec, reveal_bytes32};
use private_tx_guest::evaluate_private_tx;

fn main() {
    let input = read_vec();
    let output = evaluate_private_tx(&input);
    reveal_bytes32(output);
}

use private_tx_guest::evaluate_private_tx;
use risc0_zkvm::guest::env;

fn main() {
    let input = env::read_frame();
    let output = evaluate_private_tx(&input);
    env::commit_slice(&output);
}

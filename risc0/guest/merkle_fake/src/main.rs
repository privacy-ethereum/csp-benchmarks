use risc0_zkvm::guest::env;
use targeted_guest::evaluate_fake_merkle;

fn main() {
    let input = env::read_frame();
    let output = evaluate_fake_merkle(&input);
    env::commit_slice(&output);
}

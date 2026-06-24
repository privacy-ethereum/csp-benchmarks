use risc0_zkvm::guest::env;
use targeted_guest::evaluate_constant_overhead;

fn main() {
    let input = env::read_frame();
    let output = evaluate_constant_overhead(&input);
    env::commit_slice(&output);
}

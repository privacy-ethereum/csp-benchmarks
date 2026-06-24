use risc0_zkvm::{guest::env, sha, sha::Sha256};
use targeted_guest::evaluate_real_merkle;

fn main() {
    let input = env::read_frame();
    let output = evaluate_real_merkle(&input, |pair| {
        let digest = sha::Impl::hash_bytes(pair);
        *digest.as_bytes()
    });
    env::commit_slice(&output);
}

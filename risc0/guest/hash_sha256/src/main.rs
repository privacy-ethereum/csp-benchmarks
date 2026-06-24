use risc0_zkvm::{guest::env, sha, sha::Sha256};
use targeted_guest::evaluate_hashes;

fn main() {
    let input = env::read_frame();
    let output = evaluate_hashes(&input, |pair| {
        let digest = sha::Impl::hash_bytes(pair);
        let mut output = [0u8; 32];
        output.copy_from_slice(digest.as_bytes());
        output
    });
    env::commit_slice(&output);
}

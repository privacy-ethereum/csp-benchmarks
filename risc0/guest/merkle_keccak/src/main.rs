use risc0_zkvm::guest::env;
use targeted_guest::evaluate_real_merkle;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    let input = env::read_frame();
    let output = evaluate_real_merkle(&input, keccak);
    env::commit_slice(&output);
}

fn keccak(data: &[u8; 64]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

use risc0_zkvm::{guest::env, sha, sha::Sha256};

fn main() {
    let data = env::read_frame();
    let hash = sha::Impl::hash_bytes(&data);
    env::commit_slice(hash.as_bytes());
}

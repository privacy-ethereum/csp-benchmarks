use risc0_zkvm::{guest::env, sha, sha::Sha256};

fn main() {
    let data: Vec<u8> = env::read();
    let hash = sha::Impl::hash_bytes(&data);
    env::commit(&hash);
}

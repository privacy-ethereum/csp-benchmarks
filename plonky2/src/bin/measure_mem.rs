use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare};

fn main() {
    let (data, pw) = sha256_no_lookup_prepare();
    let _proof = prove(&data.prover_data(), pw);
}

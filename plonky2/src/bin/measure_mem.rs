use plonky2_sha256::bench::{prove, sha256_prepare};
use utils::metadata::SHA2_INPUTS;

fn main() {
    // TODO: variable input size
    let (data, pw) = sha256_prepare(SHA2_INPUTS[0]);
    let _proof = prove(&data.prover_data(), pw);
}

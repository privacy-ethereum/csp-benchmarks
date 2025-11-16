#![no_main]

use k256::{
    EncodedPoint,
    ecdsa::{Signature, VerifyingKey, signature::hazmat::PrehashVerifier},
};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    use bincode::Options;

    let input_bytes = env::read_frame();
    let (encoded_verifying_key_bytes, digest, signature_bytes): (Vec<u8>, Vec<u8>, Vec<u8>) =
        bincode::options()
            .deserialize(&input_bytes)
            .expect("Failed to deserialize input");

    let encoded_verifying_key =
        EncodedPoint::from_bytes(&encoded_verifying_key_bytes).expect("Invalid encoded point");
    let verifying_key =
        VerifyingKey::from_encoded_point(&encoded_verifying_key).expect("Invalid verifying key");
    let signature = Signature::from_slice(&signature_bytes).expect("Invalid signature");

    verifying_key
        .verify_prehash(&digest, &signature)
        .expect("ECDSA signature verification failed");

    // Commit public values as serialized tuple
    let output = (encoded_verifying_key_bytes, digest);
    let serialized = bincode::options()
        .serialize(&output)
        .expect("Failed to serialize output");
    env::commit_slice(&serialized);
}

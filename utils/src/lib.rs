use rand::{RngCore, SeedableRng, rngs::StdRng};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod bench;
pub mod harness;
pub mod metadata;
pub mod zkvm;

use p256::ecdsa::{Signature, SigningKey, signature::hazmat::PrehashSigner};

pub use harness::{BenchHarnessConfig, BenchTarget, ProvingSystem};

pub fn write_json<T: Serialize>(data: &T, output_path: &str) {
    let json_data = serde_json::to_string_pretty(&data).expect("Failed to serialize to JSON");
    let path = Path::new(&output_path);

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
}

/// Generate a random message of `input_size` bytes and its sha256 digest.
pub fn generate_sha256_input(input_size: usize) -> (Vec<u8>, Vec<u8>) {
    let mut message_bytes = vec![0u8; input_size];
    let mut rng = StdRng::seed_from_u64(input_size as u64);
    rng.fill_bytes(&mut message_bytes);

    let mut hasher = Sha256::new();
    hasher.update(&message_bytes);
    let digest_bytes = hasher.finalize().to_vec();
    (message_bytes, digest_bytes)
}

/// Generate a random secp256r1 keypair and sign a sha256 32-byte hash of a 128-byte random message.
#[allow(clippy::type_complexity)] // Allowing the return type to be "complex" for the consumers to not need to import the p256 crate
pub fn generate_ecdsa_input() -> (Vec<u8>, (Vec<u8>, Vec<u8>), Vec<u8>) {
    let mut rng = StdRng::seed_from_u64(0xecd5a);
    let signing_key = SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key().to_encoded_point(false);
    let (pub_key_x, pub_key_y) = (
        verifying_key.x().unwrap().clone().to_vec(),
        verifying_key.y().unwrap().clone().to_vec(),
    );
    let (_message, digest) = generate_sha256_input(128);
    let signature: Signature = signing_key
        .sign_prehash(&digest)
        .expect("Failed to sign prehashed digest");
    (
        digest,
        (pub_key_x, pub_key_y),
        signature.to_bytes().to_vec(),
    )
}

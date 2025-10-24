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

pub use harness::{BenchHarnessConfig, BenchTarget, ProvingSystem};

pub fn write_json<T: Serialize>(data: &T, output_path: &str) {
    let json_data = serde_json::to_string_pretty(&data).expect("Failed to serialize to JSON");
    let path = Path::new(&output_path);

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
}

pub fn generate_sha256_input(input_size: usize) -> (Vec<u8>, Vec<u8>) {
    let mut message_bytes = vec![0u8; input_size];
    let mut rng = StdRng::seed_from_u64(input_size as u64);
    rng.fill_bytes(&mut message_bytes);

    let mut hasher = Sha256::new();
    hasher.update(&message_bytes);
    let digest_bytes = hasher.finalize().to_vec();
    (message_bytes, digest_bytes)
}

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

use k256::ecdsa::{Signature as K256Signature, SigningKey as K256SigningKey};
use p256::ecdsa::{Signature, SigningKey, signature::hazmat::PrehashSigner};

pub use harness::{BenchHarnessConfig, BenchTarget, ProvingSystem};

use crate::metadata::selected_sha2_inputs;

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

/// Generate secp256r1 (p256) ECDSA test input: (digest, (pub_key_x, pub_key_y), signature).
#[allow(clippy::type_complexity)]
pub fn generate_ecdsa_input() -> (Vec<u8>, (Vec<u8>, Vec<u8>), Vec<u8>) {
    let mut rng = StdRng::seed_from_u64(0xecd5a);
    let signing_key = SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key().to_encoded_point(false);
    let (pub_key_x, pub_key_y) = (
        verifying_key.x().unwrap().to_vec(),
        verifying_key.y().unwrap().to_vec(),
    );

    let (_message, digest) = generate_sha256_input(128);
    let signature: Signature = signing_key
        .sign_prehash(&digest)
        .expect("Failed to sign prehashed digest");

    // Normalize "s" of the signature because it is not normalized by default.
    // More importantly, the "noir::std::ecdsa_secp256r1::verify_signature" expects "s" to be normalized.
    // normalize_s() returns None if the signature is already normalized, in which case we keep the original.
    let signature = signature.normalize_s().unwrap_or(signature);

    (
        digest,
        (pub_key_x, pub_key_y),
        signature.to_bytes().to_vec(),
    )
}

/// Generate secp256k1 (k256) ECDSA test input: (digest, (pub_key_x, pub_key_y), signature).
#[allow(clippy::type_complexity)]
pub fn generate_ecdsa_k256_input() -> (Vec<u8>, (Vec<u8>, Vec<u8>), Vec<u8>) {
    let mut rng = StdRng::seed_from_u64(0xecd5a);
    let signing_key = K256SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key().to_encoded_point(false);
    let (pub_key_x, pub_key_y) = (
        verifying_key.x().unwrap().to_vec(),
        verifying_key.y().unwrap().to_vec(),
    );

    let (_message, digest) = generate_sha256_input(128);
    let signature: K256Signature = signing_key
        .sign_prehash(&digest)
        .expect("Failed to sign prehashed digest");

    // Normalize "s" of the signature because it is not normalized by default.
    let signature = signature.normalize_s().unwrap_or(signature);

    (
        digest,
        (pub_key_x, pub_key_y),
        signature.to_bytes().to_vec(),
    )
}

pub fn input_sizes_for(target: BenchTarget) -> Vec<usize> {
    match target {
        BenchTarget::Sha256 => selected_sha2_inputs(),
        BenchTarget::Ecdsa => vec![32],
        BenchTarget::Keccak => selected_sha2_inputs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::EncodedPoint as K256EncodedPoint;
    use k256::ecdsa::{Signature as K256Signature, VerifyingKey as K256VerifyingKey};
    use p256::EncodedPoint;
    use p256::ecdsa::{Signature, VerifyingKey};

    #[test]
    fn test_generate_ecdsa_input_produces_valid_components() {
        let (digest, (pub_key_x, pub_key_y), signature_bytes) = generate_ecdsa_input();

        assert_eq!(pub_key_x.len(), 32);
        assert_eq!(pub_key_y.len(), 32);
        assert_eq!(signature_bytes.len(), 64);
        assert_eq!(digest.len(), 32);

        let mut encoded = Vec::with_capacity(65);
        encoded.push(0x04);
        encoded.extend_from_slice(&pub_key_x);
        encoded.extend_from_slice(&pub_key_y);

        let encoded_point =
            EncodedPoint::from_bytes(&encoded).expect("should produce valid encoded point");
        let _verifying_key = VerifyingKey::from_encoded_point(&encoded_point)
            .expect("should produce valid verifying key");
        let _signature =
            Signature::from_slice(&signature_bytes).expect("should produce valid signature");
    }

    #[test]
    fn test_ecdsa_input_is_deterministic() {
        let input1 = generate_ecdsa_input();
        let input2 = generate_ecdsa_input();
        assert_eq!(input1, input2);
    }

    #[test]
    fn test_generate_ecdsa_k256_input_produces_valid_components() {
        let (digest, (pub_key_x, pub_key_y), signature_bytes) = generate_ecdsa_k256_input();

        assert_eq!(pub_key_x.len(), 32);
        assert_eq!(pub_key_y.len(), 32);
        assert_eq!(signature_bytes.len(), 64);
        assert_eq!(digest.len(), 32);

        let mut encoded = Vec::with_capacity(65);
        encoded.push(0x04);
        encoded.extend_from_slice(&pub_key_x);
        encoded.extend_from_slice(&pub_key_y);

        let encoded_point =
            K256EncodedPoint::from_bytes(&encoded).expect("should produce valid encoded point");
        let _verifying_key = K256VerifyingKey::from_encoded_point(&encoded_point)
            .expect("should produce valid verifying key");
        let _signature =
            K256Signature::from_slice(&signature_bytes).expect("should produce valid signature");
    }

    #[test]
    fn test_ecdsa_k256_input_is_deterministic() {
        let input1 = generate_ecdsa_k256_input();
        let input2 = generate_ecdsa_k256_input();
        assert_eq!(input1, input2);
    }
}

//! ECDSA signature verification over secp256k1, as a Groth16 circuit.
//!
//! The fixed-base half uses a width-12 comb; the variable-base half is
//! verified rather than computed, with a 4-dimensional fake-GLV Straus loop
//! over a lattice hint the circuit derives for itself. The input is therefore
//! the public part of a signature and nothing else.
//!
//! The public key is validated, `r` and `s` are nonzero canonical scalars, and
//! the original prehash is reduced modulo the group order inside the circuit.
//! The shared benchmark signature is low-s normalized. The finite affine
//! additions inside the fake-GLV verifier reject exceptional additions.

use circom_prover::witness::WitnessFn;
use utils::generate_ecdsa_k256_input;

pub use crate::ecdsa_input::build_circuit_input;
pub use crate::{prove, verify};

// ECDSA witness generator
witnesscalc_adapter::witness!(ecdsa_32);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    let witness_fn = match input_size {
        32 => WitnessFn::WitnessCalc(ecdsa_32_witness),
        _ => unreachable!("Unsupported ecdsa input size: {}", input_size),
    };

    // Prepare the deterministic signature shared by the secp256k1 backends.
    let (digest, (pub_key_x, pub_key_y), signature) = generate_ecdsa_k256_input();
    let inputs = build_circuit_input(&digest, &pub_key_x, &pub_key_y, &signature);
    let input_str = serde_json::to_string(&inputs).unwrap();

    // Prepare zkey path
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let zkey_path = format!(
        "{}/circuits/ecdsa/ecdsa_{input_size}/ecdsa_{input_size}_0001.zkey",
        current_dir.as_path().to_str().unwrap()
    );

    (witness_fn, input_str, zkey_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ecdsa::{Signature, SigningKey, signature::hazmat::PrehashVerifier};

    #[test]
    fn witness_accepts_zero_prehash() {
        // d = 1, h = 0 and nonce k = 1 give Q = G, r = G.x and s = r.
        // This also makes r equal to G.x, the output of the mapped-to-one comb
        // call, so the zero branch has to bypass the ordinary same-x rejection.
        let mut secret = [0u8; 32];
        secret[31] = 1;
        let signing_key = SigningKey::from_bytes((&secret).into()).unwrap();
        let verifying_key = signing_key.verifying_key();
        let digest = [0u8; 32];
        let encoded_key = verifying_key.to_encoded_point(false);
        let r = encoded_key.x().unwrap();
        let mut signature_bytes = [0u8; 64];
        signature_bytes[..32].copy_from_slice(r);
        signature_bytes[32..].copy_from_slice(r);
        let signature = Signature::from_slice(&signature_bytes).unwrap();
        verifying_key.verify_prehash(&digest, &signature).unwrap();

        let inputs = build_circuit_input(&digest, r, encoded_key.y().unwrap(), &signature_bytes);
        let input_json = serde_json::to_string(&inputs).unwrap();

        let witness = ecdsa_32_witness(&input_json).expect("zero prehash must have a witness");
        assert!(!witness.is_empty());
    }
}

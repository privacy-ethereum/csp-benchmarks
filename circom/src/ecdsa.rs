//! ECDSA signature verification over secp256k1, as a Groth16 circuit.
//!
//! The fixed-base half uses a width-12 comb; the variable-base half is
//! verified rather than computed, with a 4-dimensional fake-GLV Straus loop
//! over the lattice hint from [`crate::glv`].
//!
//! Semantics match `k256`'s `verify_prehash`, which the other backends in this
//! repository run. One deviation, shared with `circom-ecdsa`: signatures with
//! `R.x >= n` are rejected, with probability about `2^-128`.

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

    // Prepare inputs: the same signature every other backend verifies.
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

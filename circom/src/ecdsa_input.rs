//! Building the circuit input for one secp256k1 signature.
//!
//! Kept apart from the ecdsa module because none of this needs the prover or
//! the generated witness bindings: it is the public half of a signature,
//! re-encoded into the circuit's limb layout.
//!
//! The circuit derives its own witness values — the inverse of `s`, the `y`
//! coordinate of `R`, and the lattice hint — so nothing here has to agree with
//! them. What the prover must still respect are the circuit's preconditions
//! (`R.x == r`, `r != ([u1]G).x`, a hint that fits in 64 bits); those are
//! checked in the tests below rather than on every proving run.

use k256::elliptic_curve::PrimeField;
use k256::elliptic_curve::ops::Reduce;
use k256::{FieldBytes, Scalar, U256};
use std::collections::HashMap;

/// Number of 64-bit limbs a 256-bit value is split into for the circuit.
pub const LIMBS: usize = 4;

/// Everything the circuit reads, which after the witness values moved into the
/// circuit is exactly the public part of a signature: `r`, `s`, `msghash` and
/// the public key.
pub fn build_circuit_input(
    digest: &[u8],
    pub_key_x: &[u8],
    pub_key_y: &[u8],
    signature: &[u8],
) -> HashMap<String, serde_json::Value> {
    let (r_bytes, s_bytes) = signature.split_at(32);
    let r = scalar_from_bytes(r_bytes);
    let s = scalar_from_bytes(s_bytes);
    let h = <Scalar as Reduce<U256>>::reduce_bytes(FieldBytes::from_slice(digest));

    HashMap::from([
        ("r".to_string(), limbs_json(&r.to_bytes())),
        ("s".to_string(), limbs_json(&s.to_bytes())),
        ("msghash".to_string(), limbs_json(&h.to_bytes())),
        (
            "pubkey".to_string(),
            serde_json::json!([limbs_json(pub_key_x), limbs_json(pub_key_y)]),
        ),
    ])
}

fn scalar_from_bytes(bytes: &[u8]) -> Scalar {
    Option::<Scalar>::from(Scalar::from_repr(*FieldBytes::from_slice(bytes)))
        .expect("scalar below the group order")
}

/// Big-endian 32 bytes to the circuit's four 64-bit limbs, least significant
/// first, as decimal strings — the encoding `circom-ecdsa` uses.
fn to_limbs(bytes: &[u8]) -> Vec<String> {
    let words = U256::from_be_slice(bytes).to_words();
    debug_assert_eq!(words.len(), LIMBS);
    words.iter().map(|w| w.to_string()).collect()
}

fn limbs_json(bytes: &[u8]) -> serde_json::Value {
    serde_json::json!(to_limbs(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::elliptic_curve::point::AffineCoordinates;
    use k256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
    use k256::{AffinePoint, EncodedPoint, ProjectivePoint};
    use num_bigint::BigInt;
    use num_traits::Signed;

    /// The witness values the circuit now derives for itself, recomputed here so
    /// they can be compared against the reference implementation, and so the
    /// circuit's preconditions still get asserted somewhere.
    ///
    /// Panics on a signature the circuit cannot prove: `R.x >= n`, the
    /// degenerate subtraction `r == ([u1]G).x`, or a hint needing 65 bits.
    /// Silently benchmarking an input that fails witness generation would be
    /// worse.
    struct WitnessValues {
        sinv: Vec<String>,
        r_y: Vec<String>,
        mag: Vec<String>,
        sgn: Vec<String>,
    }

    fn witness_values(
        digest: &[u8],
        pub_key_x: &[u8],
        pub_key_y: &[u8],
        signature: &[u8],
    ) -> WitnessValues {
        let (r_bytes, s_bytes) = signature.split_at(32);
        let s = scalar_from_bytes(s_bytes);
        let h = <Scalar as Reduce<U256>>::reduce_bytes(FieldBytes::from_slice(digest));

        let pubkey = affine_from_coordinates(pub_key_x, pub_key_y);

        let sinv = Option::<Scalar>::from(s.invert()).expect("s is non-zero for a valid signature");
        let u1 = h * sinv;
        let u2 = scalar_from_bytes(r_bytes) * sinv;

        // R is recomputed rather than recovered: the circuit takes R.x from the
        // public input `r` and derives only the y coordinate.
        let big_r =
            (ProjectivePoint::GENERATOR * u1 + ProjectivePoint::from(pubkey) * u2).to_affine();
        assert_ne!(big_r, AffinePoint::IDENTITY, "R must not be the identity");
        let encoded_r = big_r.to_encoded_point(false);
        let r_y = encoded_r.y().expect("R is not the identity").to_vec();
        assert_eq!(
            big_r.x().as_slice(),
            r_bytes,
            "R.x must equal r; signatures with R.x >= n are out of scope"
        );

        // Guard from step 7a of the circuit: the subtraction R - [u1]G is only
        // sound when the two operands differ.
        let u1_g = (ProjectivePoint::GENERATOR * u1).to_affine();
        assert_ne!(
            u1_g.x().as_slice(),
            r_bytes,
            "degenerate subtraction: r == ([u1]G).x"
        );

        let hint = crate::glv::decompose4(&scalar_to_bigint(&u2));
        for x in &hint {
            assert!(
                x.abs().bits() <= 64,
                "lattice hint does not fit in 64 bits: {x}"
            );
        }

        WitnessValues {
            sinv: to_limbs(&sinv.to_bytes()),
            r_y: to_limbs(&r_y),
            mag: hint.iter().map(|x| x.abs().to_string()).collect(),
            sgn: hint
                .iter()
                .map(|x| if x.is_negative() { "1" } else { "0" }.to_string())
                .collect(),
        }
    }

    fn affine_from_coordinates(x: &[u8], y: &[u8]) -> AffinePoint {
        let encoded = EncodedPoint::from_affine_coordinates(
            FieldBytes::from_slice(x),
            FieldBytes::from_slice(y),
            false,
        );
        Option::from(AffinePoint::from_encoded_point(&encoded)).expect("public key is on the curve")
    }

    fn scalar_to_bigint(s: &Scalar) -> BigInt {
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &s.to_bytes())
    }

    /// Case 0 from the phase-3 reference vectors, produced by the JavaScript
    /// implementation the circuit was witness-tested against. The circuit now
    /// derives these four values itself; the constants stay because they are
    /// what the derivation has to agree with.
    const R: &str = "96791156512790716983420836554981361349633210393766748359174448225512580277889";
    const S: &str = "33203354763501292305393002586333292996477110501749449198087039140030956875352";
    const MSGHASH: &str =
        "115036409922160129120102340064964763318413243106689753485336434403599877394652";
    const PUBKEY_X: &str =
        "63737832037230298197813806405409247279633802592228276503012513604051759695990";
    const PUBKEY_Y: &str =
        "7071887374780549439636777590279604435986066643851367285439908248767331831345";
    const EXPECTED_SINV: [&str; 4] = [
        "17727830181370692433",
        "13971681926046695686",
        "418281532449457824",
        "14210140541796473370",
    ];
    const EXPECTED_RY: [&str; 4] = [
        "1365492518906797085",
        "12802732031621248931",
        "11291563637012021028",
        "9388996153046433210",
    ];
    const EXPECTED_MAG: [&str; 4] = [
        "4831001267320049187",
        "15190723508466296597",
        "5042999121255016637",
        "1140027087195808372",
    ];
    const EXPECTED_SGN: [&str; 4] = ["0", "1", "1", "1"];

    fn be_bytes(decimal: &str) -> Vec<u8> {
        let value = BigInt::parse_bytes(decimal.as_bytes(), 10).unwrap();
        let (_, mut bytes) = value.to_bytes_be();
        while bytes.len() < 32 {
            bytes.insert(0, 0);
        }
        bytes
    }

    fn field(inputs: &HashMap<String, serde_json::Value>, key: &str) -> Vec<String> {
        serde_json::from_value(inputs[key].clone()).unwrap()
    }

    fn reference_signature() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
        let mut signature = be_bytes(R);
        signature.extend(be_bytes(S));
        (
            be_bytes(MSGHASH),
            be_bytes(PUBKEY_X),
            be_bytes(PUBKEY_Y),
            signature,
        )
    }

    #[test]
    fn input_carries_only_the_public_part() {
        let (digest, x, y, signature) = reference_signature();
        let inputs = build_circuit_input(&digest, &x, &y, &signature);

        let mut keys: Vec<&String> = inputs.keys().collect();
        keys.sort();
        assert_eq!(keys, ["msghash", "pubkey", "r", "s"]);
    }

    #[test]
    fn matches_the_javascript_witness() {
        let (digest, x, y, signature) = reference_signature();
        let values = witness_values(&digest, &x, &y, &signature);

        assert_eq!(values.sinv, EXPECTED_SINV);
        assert_eq!(values.r_y, EXPECTED_RY);
        assert_eq!(values.mag, EXPECTED_MAG);
        assert_eq!(values.sgn, EXPECTED_SGN);
    }

    #[test]
    fn limbs_are_little_endian() {
        // 2^64 + 5 -> [5, 1, 0, 0]
        let mut bytes = vec![0u8; 32];
        bytes[31] = 5;
        bytes[23] = 1;
        assert_eq!(to_limbs(&bytes), vec!["5", "1", "0", "0"]);
    }

    /// The benchmark input has to satisfy the circuit's preconditions, and
    /// nothing about the shared generator guarantees that: a signature with
    /// `R.x >= n`, or one whose hint needs 65 bits, would be rejected by the
    /// circuit rather than measured. The assertions live in `witness_values`;
    /// this test is what makes them run.
    #[test]
    fn benchmark_input_is_provable() {
        let (digest, (x, y), signature) = utils::generate_ecdsa_k256_input();
        witness_values(&digest, &x, &y, &signature);

        let inputs = build_circuit_input(&digest, &x, &y, &signature);
        for key in ["r", "s", "msghash"] {
            assert_eq!(field(&inputs, key).len(), LIMBS, "{key}");
        }
    }
}

#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{jolt, DefaultJoltMemoryConfig, JoltPlatform, Platform};
use jolt_inlines_secp256k1::{
    Secp256k1Fr, Secp256k1Point, UnwrapOrSpoilProof, ecdsa_verify,
};
use serde::Deserialize;

type Plat = JoltPlatform<DefaultJoltMemoryConfig>;

#[derive(Deserialize)]
struct EcdsaInput {
    z: [u64; 4],
    r: [u64; 4],
    s: [u64; 4],
    q: [u64; 8],
}

fn verify_ecdsa(z: [u64; 4], r: [u64; 4], s: [u64; 4], q: [u64; 8]) {
    let z = Secp256k1Fr::from_u64_arr(&z).unwrap_or_spoil_proof();
    let r = Secp256k1Fr::from_u64_arr(&r).unwrap_or_spoil_proof();
    let s = Secp256k1Fr::from_u64_arr(&s).unwrap_or_spoil_proof();
    let q = Secp256k1Point::from_u64_arr(&q).unwrap_or_spoil_proof();
    ecdsa_verify(z, r, s, q).unwrap_or_spoil_proof();
}

#[jolt::provable(guest_only)]
fn main() {
    let input_bytes = Plat::read_whole_input();
    let input: EcdsaInput = postcard::from_bytes(&input_bytes).expect("deserialize failed");

    verify_ecdsa(input.z, input.r, input.s, input.q);
    Plat::write_whole_output(&[1u8]);
}

#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{DefaultJoltMemoryConfig, JoltPlatform, Platform, jolt};
use jolt_inlines_secp256k1::{Secp256k1Fr, Secp256k1Point, UnwrapOrSpoilProof, ecdsa_verify};
use serde::Deserialize;

type Plat = JoltPlatform<DefaultJoltMemoryConfig>;

#[derive(Deserialize)]
struct EcdsaInput {
    z: [u64; 4],
    r: [u64; 4],
    s: [u64; 4],
    q: [u64; 8],
}

#[jolt::provable(guest_only)]
fn main() {
    let input_bytes = Plat::read_whole_input();
    let input: EcdsaInput = postcard::from_bytes(&input_bytes).expect("deserialize failed");

    let z = Secp256k1Fr::from_u64_arr(&input.z).unwrap_or_spoil_proof();
    let r = Secp256k1Fr::from_u64_arr(&input.r).unwrap_or_spoil_proof();
    let s = Secp256k1Fr::from_u64_arr(&input.s).unwrap_or_spoil_proof();
    let q = Secp256k1Point::from_u64_arr(&input.q).unwrap_or_spoil_proof();
    ecdsa_verify(z, r, s, q).unwrap_or_spoil_proof();
    Plat::write_whole_output(&[1u8]);
}

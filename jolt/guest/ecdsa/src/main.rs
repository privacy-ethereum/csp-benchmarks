#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{jolt, JoltMemoryConfig, JoltPlatform, Platform};
use jolt_inlines_secp256k1::{ecdsa_verify, Secp256k1Fr, Secp256k1Point};
use serde::Deserialize;

struct BenchConfig;
impl JoltMemoryConfig for BenchConfig {
    const MAX_INPUT_SIZE: u64 = 4096;
    const MAX_TRUSTED_ADVICE_SIZE: u64 = 4096;
    const MAX_UNTRUSTED_ADVICE_SIZE: u64 = 4096;
    const MAX_OUTPUT_SIZE: u64 = 4096;
    const STACK_SIZE: u64 = 4096;
    const HEAP_SIZE: u64 = 32768;
}

type Plat = JoltPlatform<BenchConfig>;

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

    let z = Secp256k1Fr::from_u64_arr(&input.z).expect("invalid z");
    let r = Secp256k1Fr::from_u64_arr(&input.r).expect("invalid r");
    let s = Secp256k1Fr::from_u64_arr(&input.s).expect("invalid s");
    let q = Secp256k1Point::from_u64_arr(&input.q).expect("invalid q");

    ecdsa_verify(z, r, s, q).expect("ECDSA verification failed");
    Plat::write_whole_output(&[1u8]);
}

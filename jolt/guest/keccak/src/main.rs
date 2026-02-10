#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{jolt, DefaultJoltMemoryConfig, JoltPlatform, Platform};
use jolt_inlines_keccak256::Keccak256;

type Plat = JoltPlatform<DefaultJoltMemoryConfig>;

#[jolt::provable(guest_only)]
fn main() {
    let input = Plat::read_whole_input();
    let output = Keccak256::digest(&*input);
    Plat::write_whole_output(&output);
}

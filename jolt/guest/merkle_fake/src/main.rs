#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{JoltMemoryConfig, JoltPlatform, Platform, jolt};
use targeted_guest::evaluate_fake_merkle;

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

#[jolt::provable(guest_only)]
fn main() {
    let input = Plat::read_whole_input();
    let output = evaluate_fake_merkle(&input);
    Plat::write_whole_output(&output);
}

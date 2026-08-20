#![no_std]
#![no_main]

extern crate alloc;

use ere_platform_jolt::{JoltMemoryConfig, JoltPlatform, Platform, jolt};
use jolt_inlines_blake3::{
    FLAG_CHUNK_END, FLAG_CHUNK_START, FLAG_PARENT, FLAG_ROOT, IV,
};

const BLOCK_LEN: usize = 64;
const CHUNK_LEN: usize = 1024;

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

#[inline(always)]
fn compress(cv: &mut [u32; 8], block: &[u8], counter: u64, block_len: u32, flags: u32) {
    let mut message = [0u32; 20];
    for (word, bytes) in message[..16].iter_mut().zip(block.chunks(4)) {
        let mut padded = [0u8; 4];
        padded[..bytes.len()].copy_from_slice(bytes);
        *word = u32::from_le_bytes(padded);
    }
    message[16] = counter as u32;
    message[17] = (counter >> 32) as u32;
    message[18] = block_len;
    message[19] = flags;

    unsafe {
        core::arch::asm!(
            ".insn r {opcode}, {funct3}, {funct7}, x0, {cv}, {message}",
            opcode = const 0x0B,
            funct3 = const 0x00,
            funct7 = const 0x03,
            cv = in(reg) cv.as_mut_ptr(),
            message = in(reg) message.as_ptr(),
            options(nostack)
        );
    }
}

fn chunk_cv(chunk: &[u8], chunk_counter: u64, root: bool) -> [u32; 8] {
    let mut cv = IV;
    let block_count = chunk.len().div_ceil(BLOCK_LEN);

    for (index, block) in chunk.chunks(BLOCK_LEN).enumerate() {
        let mut flags = 0;
        if index == 0 {
            flags |= FLAG_CHUNK_START;
        }
        if index + 1 == block_count {
            flags |= FLAG_CHUNK_END;
            if root {
                flags |= FLAG_ROOT;
            }
        }
        compress(&mut cv, block, chunk_counter, block.len() as u32, flags);
    }

    cv
}

fn parent_root(left: [u32; 8], right: [u32; 8]) -> [u32; 8] {
    let mut block = [0u8; BLOCK_LEN];
    for (index, word) in left.into_iter().chain(right).enumerate() {
        block[index * 4..index * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }

    let mut cv = IV;
    compress(&mut cv, &block, 0, BLOCK_LEN as u32, FLAG_PARENT | FLAG_ROOT);
    cv
}

fn to_bytes(words: [u32; 8]) -> [u8; 32] {
    let mut digest = [0u8; 32];
    for (index, word) in words.into_iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    digest
}

fn hash(input: &[u8]) -> [u8; 32] {
    if input.is_empty() || input.len() > 2 * CHUNK_LEN {
        panic!("BLAKE3 benchmark input must contain 1 to 2048 bytes");
    } else if input.len() <= CHUNK_LEN {
        to_bytes(chunk_cv(input, 0, true))
    } else {
        let left = chunk_cv(&input[..CHUNK_LEN], 0, false);
        let right = chunk_cv(&input[CHUNK_LEN..], 1, false);
        to_bytes(parent_root(left, right))
    }
}

#[jolt::provable(guest_only)]
fn main() {
    let input = Plat::read_whole_input();
    let output = hash(&input);
    Plat::write_whole_output(&output);
}

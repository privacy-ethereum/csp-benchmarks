#![no_std]
#![no_main]

use sha3::{Digest, Keccak256};

const INPUT_CAPACITY: usize = 4096;
const MAX_MSG_LEN: usize = INPUT_CAPACITY - 4;
static mut INPUT_BUF: [u8; MAX_MSG_LEN] = [0u8; MAX_MSG_LEN];

guest_bin::guest_main_raw!({
    // Input format: [len: u32 LE][data: u8...]
    let len = unsafe { guest_lib::io::read_input_u32() } as usize;
    let data_len = len.min(MAX_MSG_LEN);
    let buf = unsafe {
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(INPUT_BUF) as *mut u8,
            MAX_MSG_LEN,
        )
    };
    let read_len = unsafe { guest_lib::io::read_input_bytes_at(4, &mut buf[..data_len]) };

    let mut hasher = Keccak256::new();
    hasher.update(&buf[..read_len]);
    let hash: [u8; 32] = hasher.finalize().into();
    hash
});

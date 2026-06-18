#![no_std]
#![no_main]

const INPUT_CAPACITY: usize = 4096;
const MAX_INPUT_LEN: usize = INPUT_CAPACITY - 4;
static mut INPUT_BUF: [u8; MAX_INPUT_LEN] = [0u8; MAX_INPUT_LEN];

guest_bin::guest_main_raw!({
    let len = unsafe { guest_lib::io::read_input_u32() } as usize;
    let data_len = len.min(MAX_INPUT_LEN);
    let buf = unsafe {
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(INPUT_BUF) as *mut u8,
            MAX_INPUT_LEN,
        )
    };
    let read_len = unsafe { guest_lib::io::read_input_bytes_at(4, &mut buf[..data_len]) };
    assert_eq!(read_len, len);

    private_tx_guest::evaluate_private_tx(&buf[..read_len])
});

#![no_std]
#![no_main]

const INPUT_CAPACITY: usize = 16;
static mut INPUT_BUF: [u8; INPUT_CAPACITY] = [0u8; INPUT_CAPACITY];

guest_bin::guest_main_raw!({
    let len = unsafe { guest_lib::io::read_input_u32() } as usize;
    assert!(len <= INPUT_CAPACITY);
    let buf = unsafe {
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(INPUT_BUF) as *mut u8,
            INPUT_CAPACITY,
        )
    };
    let read_len = unsafe { guest_lib::io::read_input_bytes_at(4, &mut buf[..len]) };
    assert_eq!(read_len, len);
    targeted_guest::evaluate_constant_overhead(&buf[..read_len])
});

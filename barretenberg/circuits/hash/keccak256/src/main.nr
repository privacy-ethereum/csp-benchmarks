use keccak256::keccak256;

// The harness rewrites the fixed input length (both the array type and the generic)
// before compiling for each benchmark size.
fn main(input: [u8; 2048], message_size: u32) -> pub [u8; 32] {
    keccak256::<2048>(input, message_size)
}

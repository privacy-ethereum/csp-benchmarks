use sha256::sha256_var;

fn main(input: [u8; 2048], input_len: u64) -> pub [u8; 32] {
    sha256_var(input, input_len)
}

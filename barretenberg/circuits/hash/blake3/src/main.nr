fn main(input: [u8; 1024]) -> pub [u8; 32] {
    std::hash::blake3(input)
}

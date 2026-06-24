pub use crate::zkvm::hash::{PreparedHash as PreparedTargeted, build_input};

pub const CONSTANT_OVERHEAD_BENCH: &str = "constant_overhead";
pub const MERKLE_FAKE_BENCH: &str = "merkle_fake";
pub const HASH_SHA256_BENCH: &str = "hash_sha256";
pub const MERKLE_SHA256_BENCH: &str = "merkle_sha256";
pub const HASH_KECCAK_BENCH: &str = "hash_keccak";
pub const MERKLE_KECCAK_BENCH: &str = "merkle_keccak";
pub const HASH_BLAKE3_BENCH: &str = "hash_blake3";
pub const MERKLE_BLAKE3_BENCH: &str = "merkle_blake3";
pub const HASH_POSEIDON16_BENCH: &str = "hash_poseidon16";
pub const MERKLE_POSEIDON16_BENCH: &str = "merkle_poseidon16";

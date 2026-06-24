use ere_risc0::compiler::RustRv32imaCustomized;
use utils::zkvm::helpers::load_or_compile_program;
use utils::zkvm::{
    CONSTANT_OVERHEAD_BENCH, HASH_KECCAK_BENCH, HASH_SHA256_BENCH, MERKLE_FAKE_BENCH,
    MERKLE_KECCAK_BENCH, MERKLE_SHA256_BENCH, PRIVATE_TX_BENCH,
};

fn main() {
    for bench in [
        PRIVATE_TX_BENCH,
        CONSTANT_OVERHEAD_BENCH,
        MERKLE_FAKE_BENCH,
        HASH_SHA256_BENCH,
        MERKLE_SHA256_BENCH,
        HASH_KECCAK_BENCH,
        MERKLE_KECCAK_BENCH,
    ] {
        println!("Compiling {bench} guest program...");
        load_or_compile_program(&RustRv32imaCustomized, bench);
    }
    println!("Done.");
}

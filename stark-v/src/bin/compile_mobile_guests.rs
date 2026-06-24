fn main() {
    for bench in [
        "private_tx",
        "constant_overhead",
        "merkle_fake",
        "hash_sha256",
        "merkle_sha256",
        "hash_keccak",
        "merkle_keccak",
    ] {
        println!("Compiling {bench} guest program...");
        stark_v::load_or_compile(bench);
    }
    println!("Done.");
}

fn main() {
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_128");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_256");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_512");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_1024");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_2048");
}

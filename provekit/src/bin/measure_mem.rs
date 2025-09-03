use provekit::ProvekitSha256Benchmark;
use utils::metadata::SHA2_INPUTS;

fn main() {
    let input_size = SHA2_INPUTS[0];
    let bench_harness = ProvekitSha256Benchmark::new(input_size);
    let _proof = bench_harness.run_prove();
}

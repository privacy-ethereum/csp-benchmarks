use provekit::ProvekitSha256Benchmark;
use utils::metadata::SHA2_INPUTS;

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(&SHA2_INPUTS);

    for &size in SHA2_INPUTS.iter() {
        let _proof = bench_harness.run_prove(size);
    }
}

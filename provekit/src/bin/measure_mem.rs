use provekit::{ProvekitSha256Benchmark, WORKSPACE_ROOT};
use std::path::PathBuf;

const INPUT_SIZES: [usize; 1] = [2048];

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(&INPUT_SIZES);

    for &size in INPUT_SIZES.iter() {
        let _proof = bench_harness.run_prove(size);
    }
}

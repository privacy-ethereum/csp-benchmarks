use provekit::{ProvekitSha256Benchmark, WORKSPACE_ROOT};
use std::path::PathBuf;

const INPUT_SIZES: [usize; 1] = [2048];

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(&INPUT_SIZES);

    for &size in INPUT_SIZES.iter() {
        let package_name = format!("sha256_bench_{size}");
        let circuit_path = PathBuf::from(WORKSPACE_ROOT)
            .join("target")
            .join(format!("{package_name}.json"));
        let toml_path = PathBuf::from(WORKSPACE_ROOT)
            .join("circuits/hash/sha256-provekit")
            .join(format!("sha256-bench-{size}"))
            .join("Prover.toml");

        let _proof = bench_harness.run_prove(size);
    }
}

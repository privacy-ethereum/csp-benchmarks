use provekit::{ProvekitSha256Benchmark, WORKSPACE_ROOT};
use std::path::PathBuf;

const INPUT_EXPONENTS: [u32; 1] = [11];

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(&INPUT_EXPONENTS);

    let mut all_metrics = Vec::new();

    for &exp in INPUT_EXPONENTS.iter() {
        let package_name = format!("sha256_bench_2e{exp}");
        let circuit_path = PathBuf::from(WORKSPACE_ROOT)
            .join("target")
            .join(format!("{package_name}.json"));
        let toml_path = PathBuf::from(WORKSPACE_ROOT)
            .join("circuits/hash/sha256-provekit")
            .join(format!("sha256-bench-2e{exp}"))
            .join("Prover.toml");

        let _proof = bench_harness.run_prove(exp);
    }
}

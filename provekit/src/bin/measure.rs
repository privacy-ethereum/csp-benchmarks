use provekit::{ProvekitSha256Benchmark, WORKSPACE_ROOT};
use std::path::PathBuf;
use utils::bench::{SubMetrics, display_submetrics, measure_peak_memory, write_json_submetrics};

const INPUT_EXPONENTS: [u32; 1] = [11];

fn main() {
    let metrics = benchmark_sha2();
    let json_file: &'static str = "sha2_provekit_submetrics.json";
    write_json_submetrics(json_file, &metrics[0]);
}

fn benchmark_sha2() -> Vec<SubMetrics> {
    let (bench_harness, preprocessing_peak_memory) =
        measure_peak_memory(|| ProvekitSha256Benchmark::new(&INPUT_EXPONENTS));

    let mut all_metrics = Vec::new();

    for &exp in INPUT_EXPONENTS.iter() {
        let mut metrics = SubMetrics::new(1 << exp);
        metrics.preprocessing_peak_memory = preprocessing_peak_memory;

        let package_name = format!("sha256_bench_2e{exp}");
        let circuit_path = PathBuf::from(WORKSPACE_ROOT)
            .join("target")
            .join(format!("{package_name}.json"));
        let toml_path = PathBuf::from(WORKSPACE_ROOT)
            .join("circuits/hash/sha256-provekit")
            .join(format!("sha256-bench-2e{exp}"))
            .join("Prover.toml");

        metrics.preprocessing_size = std::fs::metadata(circuit_path)
            .map(|m| m.len())
            .unwrap_or(0) as usize
            + std::fs::metadata(toml_path).map(|m| m.len()).unwrap_or(0) as usize;

        let (proof, proving_peak_memory) = measure_peak_memory(|| bench_harness.run_prove(exp));
        metrics.proving_peak_memory = proving_peak_memory;
        metrics.proof_size = proof.whir_r1cs_proof.transcript.len();

        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    all_metrics
}

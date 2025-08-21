use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use provekit::{ProvekitSha256Benchmark, WORKSPACE_ROOT};
use std::path::PathBuf;
use utils::bench::{SubMetrics, display_submetrics, write_json_submetrics};
use utils::metadata::SHA2_INPUTS;

fn sha256_benchmarks(c: &mut Criterion) {
    // Measure the SubMetrics
    let metrics = sha256_submetrics();
    let json_file: &'static str = "sha2_provekit_submetrics.json";
    write_json_submetrics(json_file, &metrics[0]);

    // Run the benchmarks
    let bench_harness = ProvekitSha256Benchmark::new(&SHA2_INPUTS);
    let mut group = c.benchmark_group("SHA256 Prove & Verify");
    group.sample_size(10);

    for &input_size in SHA2_INPUTS.iter() {
        let prove_id = format!("Prove ({} bytes)", input_size);
        group.bench_function(prove_id, |bench| {
            bench.iter(|| {
                let proof = bench_harness.run_prove(input_size);
                black_box(proof);
            });
        });

        let verify_id = format!("Verify ({} bytes)", input_size);
        group.bench_function(verify_id, |bench| {
            bench.iter_batched(
                || bench_harness.prepare_verify(input_size),
                |(proof, proof_scheme)| bench_harness.run_verify(&proof, proof_scheme).unwrap(),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, sha256_benchmarks);
criterion_main!(benches);

fn sha256_submetrics() -> Vec<SubMetrics> {
    let bench_harness = ProvekitSha256Benchmark::new(&SHA2_INPUTS);

    let mut all_metrics = Vec::new();

    for &size in SHA2_INPUTS.iter() {
        let mut metrics = SubMetrics::new(size);

        let package_name = format!("sha256_bench_{size}");
        let circuit_path = PathBuf::from(WORKSPACE_ROOT)
            .join("target")
            .join(format!("{package_name}.json"));
        let toml_path = PathBuf::from(WORKSPACE_ROOT)
            .join("circuits/hash/sha256-provekit")
            .join(format!("sha256-bench-{size}"))
            .join("Prover.toml");

        metrics.preprocessing_size = std::fs::metadata(circuit_path)
            .map(|m| m.len())
            .unwrap_or(0) as usize
            + std::fs::metadata(toml_path).map(|m| m.len()).unwrap_or(0) as usize;

        let proof = bench_harness.run_prove(size);
        metrics.proof_size = proof.whir_r1cs_proof.transcript.len();

        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    all_metrics
}

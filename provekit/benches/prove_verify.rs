use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use provekit::ProvekitSha256Benchmark;
use utils::bench::{Metrics, write_json_metrics};
use utils::metadata::SHA2_INPUTS;

fn sha256_benchmarks(c: &mut Criterion) {
    for &input_size in SHA2_INPUTS.iter() {
        let bench_harness = ProvekitSha256Benchmark::new(input_size);

        // Measure the Metrics
        let metrics = sha256_provekit_metrics(&bench_harness, input_size);
        let json_file = format!("sha256_{input_size}_provekit_metrics.json");
        write_json_metrics(&json_file, &metrics);

        // Run the benchmarks
        let mut group = c.benchmark_group(format!("sha256_{input_size}_provekit"));
        group.sample_size(10);

        group.bench_function(format!("sha256_{input_size}_provekit_prove"), |bench| {
            bench.iter(|| {
                let proof = bench_harness.run_prove();
                black_box(proof);
            });
        });

        group.bench_function(format!("sha256_{input_size}_provekit_verify"), |bench| {
            bench.iter_batched(
                || bench_harness.prepare_verify(),
                |(proof, proof_scheme)| bench_harness.run_verify(&proof, proof_scheme).unwrap(),
                BatchSize::SmallInput,
            );
        });
        group.finish();
    }
}

criterion_group!(benches, sha256_benchmarks);
criterion_main!(benches);

fn sha256_provekit_metrics(bench_harness: &ProvekitSha256Benchmark, input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(
        "provekit".to_string(),
        "".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    metrics.preprocessing_size = bench_harness.preprocessing_size();

    let proof = bench_harness.run_prove();
    metrics.proof_size = proof.whir_r1cs_proof.transcript.len();

    metrics
}

use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use provekit::ProvekitSha256Benchmark;

const INPUT_EXPONENTS: [u32; 1] = [11];

fn sha256_benchmarks(c: &mut Criterion) {
    let bench_harness = ProvekitSha256Benchmark::new(&INPUT_EXPONENTS);
    let mut group = c.benchmark_group("SHA256 Prove & Verify");
    group.sample_size(10);

    for &exp in INPUT_EXPONENTS.iter() {
        let input_size = 1 << exp;
        let prove_id = format!("Prove ({} bytes)", input_size);
        group.bench_function(prove_id, |bench| {
            bench.iter(|| {
                let proof = bench_harness.run_prove(exp);
                black_box(proof);
            });
        });

        let verify_id = format!("Verify ({} bytes)", input_size);
        group.bench_function(verify_id, |bench| {
            bench.iter_batched(
                || bench_harness.prepare_verify(exp),
                |(proof, proof_scheme)| bench_harness.run_verify(&proof, proof_scheme).unwrap(),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, sha256_benchmarks);
criterion_main!(benches);

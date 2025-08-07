use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use provekit::{prove, setup, verify};

fn sha256_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_bench");
    group.sample_size(10);

    group.bench_function("sha256_bench_prove", |bench| {
        bench.iter_batched(
            || {
                let _ = setup().unwrap();
            },
            || {
                prove(2048);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_bench_verify", |bench| {
        bench.iter_batched(
            || {
                let _ = setup().unwrap();
                let proof = prove(2048);
                proof
            },
            |proof| {
                verify(2048, proof);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);

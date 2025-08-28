use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use sha::bench::{prepare_pipeline, prove, verify};
use utils::bench::{SubMetrics, write_json_submetrics};

fn sha256_bench(c: &mut Criterion) {
    // Measure the SubMetrics
    let input_size = 2048;
    let metrics = sha256_powdr_submetrics(input_size);

    let json_file = "sha256_2048_powdr_submetrics.json";
    write_json_submetrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_2048_powdr");
    group.sample_size(10);

    group.bench_function("sha256_2048_powdr_prove", |bench| {
        bench.iter_batched(
            prepare_pipeline,
            |mut pipeline| {
                prove(&mut pipeline);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_2048_powdr_verify", |bench| {
        bench.iter_batched(
            || {
                let mut pipeline = prepare_pipeline();
                prove(&mut pipeline);
                pipeline
            },
            |pipeline| {
                verify(pipeline);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);

fn sha256_powdr_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

    let mut pipeline = prepare_pipeline();

    // Load the proving key and constants from the files.
    let pk_bytes = std::fs::read("powdr-target/pkey.bin").expect("Unable to read file");
    let constants_bytes = std::fs::read("powdr-target/constants.bin").expect("Unable to read file");
    let pil_bytes = std::fs::read("powdr-target/guest.pil").expect("Unable to read file");
    metrics.preprocessing_size = pk_bytes.len() + constants_bytes.len() + pil_bytes.len();

    let _ = prove(&mut pipeline);
    metrics.proof_size = pipeline.proof().unwrap().len();

    metrics
}

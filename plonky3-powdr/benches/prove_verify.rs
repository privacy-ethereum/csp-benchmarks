use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use sha::bench::{prepare_pipeline, prove, verify};
use utils::{
    bench::{SubMetrics, display_submetrics, measure_peak_memory, write_json_submetrics},
    metadata::SHA2_INPUTS,
};

fn sha256_bench(c: &mut Criterion) {
    let mut all_metrics = Vec::new();

    for &num_byte in SHA2_INPUTS.iter() {
        let metrics = sha2_plonky3_powdr_submetrics(num_byte);
        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    let json_path = "sha2_plonky3_powdr_submetrics.json";
    write_json_submetrics(json_path, &all_metrics[0]);

    let mut group = c.benchmark_group("sha256_bench");
    group.sample_size(10);

    group.bench_function("sha256_bench_prove", |bench| {
        bench.iter_batched(
            prepare_pipeline,
            |mut pipeline| {
                prove(&mut pipeline);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_bench_verify", |bench| {
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

fn sha2_plonky3_powdr_submetrics(num_bytes: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(num_bytes);

    let (mut pipeline, peak_memory) = measure_peak_memory(prepare_pipeline);
    metrics.preprocessing_peak_memory = peak_memory;

    // Load the proving key and constants from the files.
    let pk_bytes = std::fs::read("powdr-target/pkey.bin").expect("Unable to read file");
    let constants_bytes = std::fs::read("powdr-target/constants.bin").expect("Unable to read file");
    let pil_bytes = std::fs::read("powdr-target/guest.pil").expect("Unable to read file");
    metrics.preprocessing_size = pk_bytes.len() + constants_bytes.len() + pil_bytes.len();

    let (_, peak_memory) = measure_peak_memory(|| prove(&mut pipeline));
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = pipeline.proof().unwrap().len();

    metrics
}

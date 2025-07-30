use sha::bench::{prepare_pipeline, prove, verify};
use std::time::Instant;
use utils::bench::{CustomMetrics, measure_peak_memory, write_csv_custom};

fn main() {
    let csv_file = "sha2_plonky3_powdr.csv";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_csv_custom(csv_file, &[metrics]);
}

fn benchmark_sha2(num_bytes: usize) -> CustomMetrics {
    let mut metrics = CustomMetrics::new(num_bytes);

    let (mut pipeline, peak_memory) = measure_peak_memory(|| prepare_pipeline());
    metrics.preprocessing_peak_memory = peak_memory;
    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics.preprocessing_size = 0; // TODO

    let start = Instant::now();
    let (_, peak_memory) = measure_peak_memory(|| prove(&mut pipeline));
    metrics.proof_duration = start.elapsed();
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = pipeline.proof().unwrap().len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    let start = Instant::now();
    verify(pipeline);
    metrics.verify_duration = start.elapsed();

    metrics
}

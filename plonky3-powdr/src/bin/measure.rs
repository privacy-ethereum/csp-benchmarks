use sha::bench::{prepare_pipeline, prove};
use utils::bench::{SubMetrics, measure_peak_memory, write_json_submetrics};

fn main() {
    let json_file = "sha2_plonky3_powdr_submetrics.json";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_json_submetrics(json_file, &metrics);
}

fn benchmark_sha2(num_bytes: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(num_bytes);

    let (mut pipeline, peak_memory) = measure_peak_memory(|| prepare_pipeline());
    metrics.preprocessing_peak_memory = peak_memory;
    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics.preprocessing_size = 0; // TODO

    let (_, peak_memory) = measure_peak_memory(|| prove(&mut pipeline));
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = pipeline.proof().unwrap().len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics
}

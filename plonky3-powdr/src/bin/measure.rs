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

    let (mut pipeline, peak_memory) = measure_peak_memory(prepare_pipeline);
    metrics.preprocessing_peak_memory = peak_memory;
    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    // Load the proving key and constants from the files.
    let pk_bytes = std::fs::read("powdr-target/pkey.bin").expect("Unable to read file");
    let constants_bytes = std::fs::read("powdr-target/constants.bin").expect("Unable to read file");
    let pil_bytes = std::fs::read("powdr-target/guest.pil").expect("Unable to read file");
    metrics.preprocessing_size = pk_bytes.len() + constants_bytes.len() + pil_bytes.len();

    let (_, peak_memory) = measure_peak_memory(|| prove(&mut pipeline));
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = pipeline.proof().unwrap().len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics
}

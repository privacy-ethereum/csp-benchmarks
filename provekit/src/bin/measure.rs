use provekit::{CIRCUIT_ROOT, prove, setup};
use utils::bench::{SubMetrics, measure_peak_memory, write_json_submetrics};

fn main() {
    let json_file = "sha2_provekit_submetrics.json";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_json_submetrics(json_file, &metrics);
}

fn benchmark_sha2(input_num_bytes: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_num_bytes);

    let (_, peak_memory) = measure_peak_memory(|| setup().unwrap());
    metrics.preprocessing_peak_memory = peak_memory;
    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    // Load the preprocessing files.
    let circuit_dir = format!("{}/sha256-bench-{}", CIRCUIT_ROOT, input_num_bytes);
    let circuit_path = format!("circuits/target/sha256_bench_{}.json", input_num_bytes);
    let prover_toml_path = format!("{}/Prover.toml", circuit_dir);

    let proof_scheme_file_bytes = std::fs::read(&circuit_path).unwrap();
    let input_map_file_bytes = std::fs::read(&prover_toml_path).unwrap();
    metrics.preprocessing_size = proof_scheme_file_bytes.len() + input_map_file_bytes.len();

    let (proof, peak_memory) = measure_peak_memory(|| prove(input_num_bytes));
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof.whir_r1cs_proof.transcript.len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics
}

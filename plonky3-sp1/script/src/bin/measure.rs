use sp1_sdk::{ProverClient, SP1Stdin, include_elf};
use utils::bench::{SubMetrics, measure_peak_memory, write_json_submetrics};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn main() {
    let json_file = "sha2_plonky3_sp1_submetrics.json";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_json_submetrics(json_file, &metrics);
}

fn benchmark_sha2(input_num_bytes: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_num_bytes);

    // Load the proving key and verifying key from the files.
    let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
    let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
    // Load the proof from the file.
    let proof_bytes = std::fs::read("proof.bin").expect("Unable to read file");

    // Setup the prover client.
    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();

    // Setup the program for proving.
    let ((_, _), peak_memory) = measure_peak_memory(|| client.setup(SHA_ELF));

    metrics.preprocessing_peak_memory = peak_memory;
    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    metrics.preprocessing_size = pk_bytes.len() + SHA_ELF.len(); // correct?

    // Generate the proof
    let (_, peak_memory) = measure_peak_memory(|| {
        client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof")
    });
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof_bytes.len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    metrics
}

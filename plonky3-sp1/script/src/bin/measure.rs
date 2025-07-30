use sp1_sdk::{ProverClient, SP1Stdin, include_elf};
use std::time::Instant;
use utils::bench::{CustomMetrics, measure_peak_memory, write_csv_custom};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn main() {
    let csv_file = "sha2_plonky3_sp1.csv";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_csv_custom(csv_file, &[metrics]);
}

fn benchmark_sha2(input_num_bytes: usize) -> CustomMetrics {
    let mut metrics = CustomMetrics::new(input_num_bytes);

    // Load the proving key and verifying key from the files.
    let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
    let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
    // Load the verifying key from the file.
    let vk_bytes = std::fs::read("vk.bin").expect("Unable to read file");
    let vk: sp1_sdk::SP1VerifyingKey = bincode::deserialize(&vk_bytes).unwrap();
    // Load the proof from the file.
    let proof_bytes = std::fs::read("proof.bin").expect("Unable to read file");
    let proof: sp1_sdk::SP1ProofWithPublicValues = bincode::deserialize(&proof_bytes).unwrap();

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

    metrics.preprocessing_size = pk_bytes.len() + SHA_ELF.len(); // TODO

    // Generate the proof
    let start = Instant::now();
    let (_, peak_memory) = measure_peak_memory(|| {
        client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof")
    });
    metrics.proof_duration = start.elapsed();
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof_bytes.len();

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    // Verify the proof
    let start = Instant::now();
    client.verify(&proof, &vk).expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}

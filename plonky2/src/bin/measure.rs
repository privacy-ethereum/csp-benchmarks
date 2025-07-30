use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare, verify};
use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};

use std::time::Instant;
use utils::bench::{CustomMetrics, measure_peak_memory, write_csv_custom};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

fn main() {
    let csv_file = "sha2_plonky2.csv";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes);

    write_csv_custom(csv_file, &[metrics]);
}

fn benchmark_sha2(num_bytes: usize) -> CustomMetrics {
    let mut metrics = CustomMetrics::new(num_bytes);

    let ((data, pw), peak_memory) = measure_peak_memory(|| sha256_no_lookup_prepare());
    metrics.preprocessing_peak_memory = peak_memory;

    let gate_serializer = U32GateSerializer;
    let common_data_size = data.common.to_bytes(&gate_serializer).unwrap().len();
    let generator_serializer = U32GeneratorSerializer::<C, D>::default();
    let prover_data_size = data
        .prover_only
        .to_bytes(&generator_serializer, &data.common)
        .unwrap()
        .len();
    let verifier_data = data.verifier_data();

    println!(
        "Common data size: {}B, Prover data size: {}B",
        common_data_size, prover_data_size
    );
    metrics.preprocessing_size = prover_data_size; // TODO

    let start = Instant::now();
    let (proof, peak_memory) = measure_peak_memory(|| prove(&data.prover_data(), pw));
    metrics.proof_duration = start.elapsed();
    metrics.proving_peak_memory = peak_memory;

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    let mut buffer = Vec::new();
    buffer.write_proof(&proof.proof).unwrap();
    println!("Proof size: {} KB", buffer.len() as f32 / 1024.0);
    metrics.proof_size = buffer.len();

    let start = Instant::now();
    verify(&verifier_data, proof);
    metrics.verify_duration = start.elapsed();

    metrics
}

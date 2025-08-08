use anyhow::Error;
use binius::bench::{prove, sha256_with_lookup_prepare};
use binius_utils::SerializeBytes;
use utils::bench::{SubMetrics, measure_peak_memory, write_json_submetrics};

fn main() -> Result<(), Error> {
    let json_file = "sha2_binius_lookup_submetrics.json";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes)?;

    write_json_submetrics(json_file, &metrics);

    Ok(())
}

fn benchmark_sha2(num_bytes: usize) -> Result<SubMetrics, Error> {
    let mut metrics = SubMetrics::new(num_bytes);

    let allocator = bumpalo::Bump::new();

    let ((constraint_system, args, witness, backend), peak_memory) =
        measure_peak_memory(|| sha256_with_lookup_prepare(&allocator));
    metrics.preprocessing_peak_memory = peak_memory;

    let mut buffer: Vec<u8> = Vec::new();
    constraint_system
        .serialize(&mut buffer, binius_utils::SerializationMode::CanonicalTower)
        .expect("Failed to serialize constraint system");
    metrics.preprocessing_size = buffer.len();

    let ((_, _, proof), peak_memory) =
        measure_peak_memory(|| prove(constraint_system, args, witness, backend));
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof.get_proof_size();

    Ok(metrics)
}

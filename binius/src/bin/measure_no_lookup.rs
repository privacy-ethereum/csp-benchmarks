use anyhow::Error;
use binius::bench::{prove, sha256_no_lookup_prepare, verify};
use std::time::Instant;
use utils::bench::{CustomMetrics, measure_peak_memory, write_csv_custom};

fn main() -> Result<(), Error> {
    let csv_file = "sha2_binius_no_lookup.csv";

    let input_num_bytes = 2048;
    let metrics = benchmark_sha2(input_num_bytes)?;

    write_csv_custom(csv_file, &[metrics]);

    Ok(())
}

fn benchmark_sha2(num_bytes: usize) -> Result<CustomMetrics, Error> {
    let mut metrics = CustomMetrics::new(num_bytes);

    let allocator = bumpalo::Bump::new();

    let ((constraint_system, args, witness, backend), peak_memory) =
        measure_peak_memory(|| sha256_no_lookup_prepare(&allocator));
    metrics.preprocessing_peak_memory = peak_memory;
    metrics.preprocessing_size = 0; // TODO

    let start = Instant::now();
    let ((cs, args, proof), peak_memory) =
        measure_peak_memory(|| prove(constraint_system, args, witness, backend));
    metrics.proof_duration = start.elapsed();
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof.get_proof_size();

    let start = Instant::now();
    verify(cs, args, proof);
    metrics.verify_duration = start.elapsed();

    Ok(metrics)
}

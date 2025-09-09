use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;
use utils::bench::Metrics;
use utils::bench::write_json_metrics;
use utils::metadata::SHA2_INPUTS;

fn criterion_benchmarks(c: &mut Criterion) {
    for input_size in SHA2_INPUTS {
        let universe = MPIConfig::init().expect("Failed to initialize MPI");
        let world = universe.world();
        let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));

        // Measure the metrics
        let metrics = sha256_expander_metrics(input_size, mpi_config.clone());

        let json_file = format!("sha256_{input_size}_expander_metrics.json");
        write_json_metrics(&json_file, &metrics);

        let mut group = c.benchmark_group("sha256_expander");
        group.sample_size(10);

        // Proving benchmark
        group.bench_function(format!("sha256_{input_size}_expander_prove"), |bench| {
            bench.iter_batched(
                || prepare(input_size),
                |(circuit_file, witness_file)| {
                    prove(&circuit_file, &witness_file, mpi_config.clone());
                },
                BatchSize::SmallInput,
            );
        });

        // Verify benchmark
        group.bench_function(format!("sha256_{input_size}_expander_verify"), |bench| {
            bench.iter_batched(
                || {
                    // Prepare & prove to obtain proof for verification
                    let (circuit_file, witness_file) = prepare(input_size);

                    let (claimed_v, proof) =
                        prove(&circuit_file, &witness_file, mpi_config.clone());

                    (
                        circuit_file,
                        witness_file,
                        claimed_v,
                        proof,
                        mpi_config.clone(),
                    )
                },
                |(circuit_file, witness_file, claimed, proof, mpi_config)| {
                    // Set up verifier

                    verify(&circuit_file, &witness_file, &proof, &claimed, mpi_config);
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn sha256_expander_metrics(input_size: usize, mpi_config: MPIConfig<'_>) -> Metrics {
    let mut metrics = Metrics::new(
        "expander".to_string(),
        "".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    let (circuit_file, witness_file) = prepare(input_size);

    metrics.preprocessing_size = std::fs::metadata(&circuit_file).unwrap().len() as usize;

    let (_, proof) = prove(&circuit_file, &witness_file, mpi_config.clone());
    metrics.proof_size = proof.bytes.len();

    metrics
}

criterion_group!(sha256_expander_benches, criterion_benchmarks);
criterion_main!(sha256_expander_benches);

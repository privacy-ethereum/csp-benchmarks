// Copyright 2024-2025 Irreducible Inc.

use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_utils::serialization::SerializeBytes;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::{prepare, prove, verify};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use utils::{
    bench::{Metrics, compile_binary, run_measure_mem_script, write_json_metrics},
    metadata::SHA2_INPUTS,
};

fn sha256_bench(c: &mut Criterion) {
    for input_size in SHA2_INPUTS {
        // Measure the metrics
        let metrics = sha256_binius64_metrics(input_size);

        let json_file = format!("sha256_{input_size}_binius64_metrics.json");
        write_json_metrics(&json_file, &metrics);

        // RAM measurement
        let sha256_binary_name = "sha256_mem";
        compile_binary(sha256_binary_name);

        let sha256_binary_path = format!("../target/release/{}", sha256_binary_name);
        let json_file = format!("sha256_{}_binius64_mem_report.json", input_size);
        run_measure_mem_script(&json_file, &sha256_binary_path, input_size);

        // Run the (criterion) benchmarks
        let mut group = c.benchmark_group(format!("sha256_{input_size}_binius64"));
        group.sample_size(10);

        group.bench_function(format!("sha256_{input_size}_binius64_prove"), |bench| {
            bench.iter_batched(
                || prepare(input_size).expect("Failed to prepare sha256 circuit for prove/verify"),
                |(_, prover, witness, _)| {
                    prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
                        &prover, witness,
                    )
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(format!("sha256_{input_size}_binius64_verify"), |bench| {
            bench.iter_batched(
                || {
                    let (verifier, prover, witness, _) = prepare(input_size)
                        .expect("Failed to prepare sha256 circuit for prove/verify");
                    let proof = prove::<
                        StdDigest,
                        StdCompression,
                        ParallelCompressionAdaptor<StdCompression>,
                    >(&prover, witness.clone())
                    .expect("Failed to prove sha256 circuit");
                    (verifier, proof, witness)
                },
                |(verifier, proof, witness)| {
                    verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
                        &verifier, witness, &proof,
                    )
                },
                BatchSize::SmallInput,
            );
        });
        group.finish();
    }
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);

fn sha256_binius64_metrics(input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(
        "binius64".to_string(),
        "".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    let (_verifier, prover, witness, cs) =
        prepare(input_size).expect("Failed to prepare sha256 circuit for prove/verify");

    let mut buf: Vec<u8> = Vec::new();
    cs.serialize(&mut buf)
        .expect("Failed to serialize constraint system into byte array");

    metrics.preprocessing_size = buf.len();

    let proof = prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
        &prover, witness,
    )
    .expect("Failed to prove sha256 circuit");
    metrics.proof_size = proof.len();

    metrics
}

// Copyright 2024-2025 Irreducible Inc.

use binius::bench::{prove, sha256_no_lookup_prepare, sha256_with_lookup_prepare, verify};
use binius_utils::SerializeBytes;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use utils::bench::{Metrics, write_json_metrics};

fn sha256_no_lookup(c: &mut Criterion) {
    // Measure the metrics
    let input_size = 2048;
    let metrics = sha256_binius_no_lookup_metrics(input_size);

    let json_file = "sha256_2048_binius_no_lookup_metrics.json";
    write_json_metrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_2048_binius_no_lookup");
    group.sample_size(10);
    let allocator = bumpalo::Bump::new();

    group.bench_function("sha256_2048_binius_no_lookup_prove", |bench| {
        bench.iter_batched(
            || sha256_no_lookup_prepare(&allocator),
            |(constraint_system, args, witness, backend)| {
                prove(constraint_system, args, witness, backend);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_2048_binius_no_lookup_verify", |bench| {
        bench.iter_batched(
            || {
                let (constraint_system, args, witness, backend) =
                    sha256_no_lookup_prepare(&allocator);
                prove(constraint_system, args, witness, backend)
            },
            |(constraint_system, args, proof)| {
                verify(constraint_system, args, proof);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn sha256_with_lookup(c: &mut Criterion) {
    // Measure the metrics
    let input_size = 2048;
    let metrics = sha256_binius_with_lookup_metrics(input_size);

    let json_file = "sha256_2048_binius_with_lookup_metrics.json";
    write_json_metrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_2048_binius_with_lookup");
    group.sample_size(10);
    let allocator = bumpalo::Bump::new();

    group.bench_function("sha256_2048_binius_with_lookup_prove", |bench| {
        bench.iter_batched(
            || sha256_with_lookup_prepare(&allocator),
            |(constraint_system, args, witness, backend)| {
                prove(constraint_system, args, witness, backend);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_2048_binius_with_lookup_verify", |bench| {
        bench.iter_batched(
            || {
                let (constraint_system, args, witness, backend) =
                    sha256_with_lookup_prepare(&allocator);
                prove(constraint_system, args, witness, backend)
            },
            |(constraint_system, args, proof)| {
                verify(constraint_system, args, proof);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_no_lookup, sha256_with_lookup);

fn sha256_binius_with_lookup_metrics(input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(
        "binius".to_string(),
        "with_lookup".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) = sha256_with_lookup_prepare(&allocator);

    let mut buffer: Vec<u8> = Vec::new();
    constraint_system
        .serialize(&mut buffer, binius_utils::SerializationMode::CanonicalTower)
        .expect("Failed to serialize constraint system");
    metrics.preprocessing_size = buffer.len();

    let (_, _, proof) = prove(constraint_system, args, witness, backend);
    metrics.proof_size = proof.get_proof_size();

    metrics
}

fn sha256_binius_no_lookup_metrics(input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(
        "binius".to_string(),
        "no_lookup".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) = sha256_no_lookup_prepare(&allocator);

    let mut buffer: Vec<u8> = Vec::new();
    constraint_system
        .serialize(&mut buffer, binius_utils::SerializationMode::CanonicalTower)
        .expect("Failed to serialize constraint system");
    metrics.preprocessing_size = buffer.len();

    let (_, _, proof) = prove(constraint_system, args, witness, backend);
    metrics.proof_size = proof.get_proof_size();

    metrics
}

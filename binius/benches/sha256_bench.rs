// Copyright 2024-2025 Irreducible Inc.

use binius::bench::{prove, sha256_no_lookup_prepare, sha256_with_lookup_prepare, verify};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use binius_utils::SerializeBytes;
use utils::bench::{SubMetrics, write_json_submetrics};

fn sha256_no_lookup(c: &mut Criterion) {
    // Measure the SubMetrics
    let input_size = 2048;
    let metrics = sha2_no_lookup_submetrics(input_size);
    
    let json_file = "sha2_binius_no_lookup_submetrics.json";
    write_json_submetrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_no_lookup");
    group.sample_size(10);
    let allocator = bumpalo::Bump::new();

    group.bench_function("sha256_no_lookup_prove", |bench| {
        bench.iter_batched(
            || sha256_no_lookup_prepare(&allocator),
            |(constraint_system, args, witness, backend)| {
                prove(constraint_system, args, witness, backend);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_no_lookup_verify", |bench| {
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
    // Measure the SubMetrics
    let input_size = 2048;
    let metrics = sha2_with_lookup_submetrics(input_size);
    
    let json_file = "sha2_binius_lookup_submetrics.json";
    write_json_submetrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_with_lookup");
    group.sample_size(10);
    let allocator = bumpalo::Bump::new();

    group.bench_function("sha256_with_lookup_prove", |bench| {
        bench.iter_batched(
            || sha256_with_lookup_prepare(&allocator),
            |(constraint_system, args, witness, backend)| {
                prove(constraint_system, args, witness, backend);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_with_lookup_verify", |bench| {
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

fn sha2_with_lookup_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) =
        sha256_with_lookup_prepare(&allocator);

    let mut buffer: Vec<u8> = Vec::new();
    constraint_system
        .serialize(&mut buffer, binius_utils::SerializationMode::CanonicalTower)
        .expect("Failed to serialize constraint system");
    metrics.preprocessing_size = buffer.len();

    let (_, _, proof) = prove(constraint_system, args, witness, backend);
    metrics.proof_size = proof.get_proof_size();

    metrics
}

fn sha2_no_lookup_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) =
        sha256_no_lookup_prepare(&allocator);

    let mut buffer: Vec<u8> = Vec::new();
    constraint_system
        .serialize(&mut buffer, binius_utils::SerializationMode::CanonicalTower)
        .expect("Failed to serialize constraint system");
    metrics.preprocessing_size = buffer.len();

    let (_, _, proof) = prove(constraint_system, args, witness, backend);
    metrics.proof_size = proof.get_proof_size();

    metrics
}

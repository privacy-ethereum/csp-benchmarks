// Copyright 2024-2025 Irreducible Inc.

use binius::bench::{prove, sha256_no_lookup_prepare, sha256_with_lookup_prepare, verify};
use binius_utils::SerializeBytes;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use utils::{
    bench::{SubMetrics, display_submetrics, measure_peak_memory, write_json_submetrics},
    metadata::SHA2_INPUTS,
};

fn sha256_no_lookup(c: &mut Criterion) {
    let mut all_metrics = Vec::new();

    for &num_byte in SHA2_INPUTS.iter() {
        let metrics = sha2_binius_no_lookup_submetrics(num_byte);
        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    let json_path = "sha2_binius_no_lookup_submetrics.json";
    write_json_submetrics(json_path, &all_metrics[0]);

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
    let mut all_metrics = Vec::new();

    for &input_size in SHA2_INPUTS.iter() {
        let metrics = sha2_binius_with_lookup_submetrics(input_size);
        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    let json_path = "sha2_binius_lookup_submetrics.json";
    write_json_submetrics(json_path, &all_metrics[0]);

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

fn sha2_binius_no_lookup_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

    let allocator = bumpalo::Bump::new();

    let ((constraint_system, args, witness, backend), peak_memory) =
        measure_peak_memory(|| sha256_no_lookup_prepare(&allocator));
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

    metrics
}

fn sha2_binius_with_lookup_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

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

    metrics
}

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare, verify};

use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};
use utils::bench::{Metrics, write_json_metrics};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

fn sha256_no_lookup(c: &mut Criterion) {
    // Measure the metrics
    let input_size = 2048;
    let metrics = sha256_plonky2_no_lookup_metrics(input_size);

    let json_file = "sha256_2048_plonky2_no_lookup_metrics.json";
    write_json_metrics(json_file, &metrics);

    // Run the benchmarks
    let mut group = c.benchmark_group("sha256_2048_plonky2_no_lookup");
    group.sample_size(10);

    group.bench_function("sha256_2048_plonky2_no_lookup_prove", |bench| {
        bench.iter_batched(
            sha256_no_lookup_prepare,
            |(data, pw)| {
                prove(&data.prover_data(), pw);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_2048_plonky2_no_lookup_verify", |bench| {
        bench.iter_batched(
            || {
                let (data, pw) = sha256_no_lookup_prepare();
                let verifier_data = data.verifier_data();
                (prove(&data.prover_data(), pw), verifier_data)
            },
            |(proof, data)| {
                verify(&data, proof);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_no_lookup);

fn sha256_plonky2_no_lookup_metrics(input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(
        "plonky2".to_string(),
        "no_lookup".to_string(),
        false,
        "sha256".to_string(),
        input_size,
    );

    let (data, pw) = sha256_no_lookup_prepare();

    let gate_serializer = U32GateSerializer;
    let common_data_size = data.common.to_bytes(&gate_serializer).unwrap().len();
    let generator_serializer = U32GeneratorSerializer::<C, D>::default();
    let prover_data_size = data
        .prover_only
        .to_bytes(&generator_serializer, &data.common)
        .unwrap()
        .len();

    println!(
        "Common data size: {}B, Prover data size: {}B",
        common_data_size, prover_data_size
    );
    metrics.preprocessing_size = prover_data_size + common_data_size;

    let proof = prove(&data.prover_data(), pw);

    let mut buffer = Vec::new();
    buffer.write_proof(&proof.proof).unwrap();
    metrics.proof_size = buffer.len();

    metrics
}

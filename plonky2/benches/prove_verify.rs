use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare, verify};

use utils::{
    bench::{SubMetrics, display_submetrics, measure_peak_memory, write_json_submetrics},
    metadata::SHA2_INPUTS,
};

fn sha256_no_lookup(c: &mut Criterion) {
    let mut all_metrics = Vec::new();

    for &num_byte in SHA2_INPUTS.iter() {
        let metrics = sha2_plonky2_submetrics(num_byte);
        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    let json_path = "sha2_plonky2_submetrics.json";
    write_json_submetrics(json_path, &all_metrics[0]);

    let mut group = c.benchmark_group("sha256_no_lookup");
    group.sample_size(10);

    group.bench_function("sha256_no_lookup_prove", |bench| {
        bench.iter_batched(
            sha256_no_lookup_prepare,
            |(data, pw)| {
                prove(&data.prover_data(), pw);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_no_lookup_verify", |bench| {
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

fn sha2_plonky2_submetrics(num_bytes: usize) -> SubMetrics {
    use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
    use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare};
    use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;

    let mut metrics = SubMetrics::new(num_bytes);

    let ((data, pw), peak_memory) = measure_peak_memory(sha256_no_lookup_prepare);
    metrics.preprocessing_peak_memory = peak_memory;

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

    let (proof, peak_memory) = measure_peak_memory(|| prove(&data.prover_data(), pw));
    metrics.proving_peak_memory = peak_memory;

    let mut buffer = Vec::new();
    buffer.write_proof(&proof.proof).unwrap();
    metrics.proof_size = buffer.len();

    metrics
}

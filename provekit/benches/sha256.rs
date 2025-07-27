use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use noir_r1cs::NoirProofScheme;
use std::time::Duration;

// TODO: Add RAM usage metrics using G's utils
// TODO: Measure proof size
// TODO: Implement measurement's matrix
// TODO: Create CSV from measurements 

const CIRCUIT_SIZES: [u32; 4] = [8, 10, 12, 14];

fn bench_proof_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_proof_generation");
    group.measurement_time(Duration::from_secs(120));
    group.sample_size(10);

    for exp in CIRCUIT_SIZES {
        let size_label = format!("2^{}", exp);
        let circuit_path = format!("circuits/target/sha256_bench_2e{}.json", exp);
        let prover_toml_path = format!(
            "circuits/hash/sha256-provekit/sha256-bench-2e{}/Prover.toml",
            exp
        );
        let size = 1usize << exp;

        println!("Starting proof generation benchmark for {}", size_label);

        let proof_scheme = NoirProofScheme::from_file(&circuit_path).unwrap();
        let input_map = proof_scheme.read_witness(&prover_toml_path).unwrap();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("prove", size_label),
            &(&proof_scheme, &input_map),
            |b, (scheme, input_map)| {
                b.iter(|| {
                    let proof = scheme.prove(input_map).unwrap();
                    proof
                })
            },
        );
    }

    group.finish();
}

fn bench_proof_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_proof_verification");
    group.measurement_time(Duration::from_secs(30));

    for exp in CIRCUIT_SIZES {
        let size_label = format!("2^{}", exp);
        let circuit_path = format!("circuits/target/sha256_bench_2e{}.json", exp);
        let prover_toml_path = format!(
            "circuits/hash/sha256-provekit/sha256-bench-2e{}/Prover.toml",
            exp
        );
        let size = 1usize << exp;

        println!("Starting proof verification benchmark for {}", size_label);

        let proof_scheme = NoirProofScheme::from_file(&circuit_path).unwrap();
        let input_map = proof_scheme.read_witness(&prover_toml_path).unwrap();
        let proof = proof_scheme.prove(&input_map).unwrap();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("verify", size_label),
            &(&proof_scheme, &proof),
            |b, (scheme, proof)| b.iter(|| scheme.verify(proof).unwrap()),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_proof_generation, bench_proof_verification);
criterion_main!(benches);

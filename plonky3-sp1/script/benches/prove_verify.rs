use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use sp1_sdk::{EnvProver, ProverClient, SP1ProvingKey, SP1Stdin, SP1VerifyingKey, include_elf};
use utils::{
    bench::{SubMetrics, display_submetrics, measure_peak_memory, write_json_submetrics},
    metadata::SHA2_INPUTS,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn sha256_prepare(client: &EnvProver) -> (SP1ProvingKey, SP1VerifyingKey) {
    client.setup(SHA_ELF)
}

fn sha256_bench(c: &mut Criterion) {
    let mut all_metrics = Vec::new();

    for &input_size in SHA2_INPUTS.iter() {
        let metrics = sha2_plonky3_sp1_submetrics(input_size);
        all_metrics.push(metrics);
    }

    println!("{}", display_submetrics(&all_metrics));

    let json_path = "sha2_plonky3_sp1_submetrics.json";
    write_json_submetrics(json_path, &all_metrics[0]);

    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();
    // // Setup the program for proving.
    // let (pk, vk) = client.setup(SHA_ELF);

    // // Generate the proof
    // let proof = client
    //     .prove(&pk, &stdin)
    //     .run()
    //     .expect("failed to generate proof");

    // println!("Successfully generated proof!");

    // // Verify the proof.
    // client.verify(&proof, &vk).expect("failed to verify proof");
    // println!("Successfully verified proof!");

    let mut group = c.benchmark_group("sha256_bench");
    group.sample_size(10);

    group.bench_function("sha256_bench_prove", |bench| {
        bench.iter_batched(
            || sha256_prepare(&client),
            |(pk, _pw)| {
                client
                    .prove(&pk, &stdin)
                    .run()
                    .expect("failed to generate proof");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_bench_verify", |bench| {
        bench.iter_batched(
            || {
                let (pk, vk) = sha256_prepare(&client);
                (
                    client
                        .prove(&pk, &stdin)
                        .run()
                        .expect("failed to generate proof"),
                    vk,
                )
            },
            |(proof, vk)| {
                client.verify(&proof, &vk).expect("failed to verify proof");
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);

fn sha2_plonky3_sp1_submetrics(input_size: usize) -> SubMetrics {
    let mut metrics = SubMetrics::new(input_size);

    // Load the proving key and verifying key from the files.
    let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
    let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
    // Load the proof from the file.
    let proof_bytes = std::fs::read("proof.bin").expect("Unable to read file");

    // Setup the prover client.
    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();

    // Setup the program for proving.
    let ((_, _), peak_memory) = measure_peak_memory(|| client.setup(SHA_ELF));

    metrics.preprocessing_peak_memory = peak_memory;
    metrics.preprocessing_size = pk_bytes.len() + SHA_ELF.len();

    // Generate the proof
    let (_, peak_memory) = measure_peak_memory(|| {
        client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof")
    });
    metrics.proving_peak_memory = peak_memory;
    metrics.proof_size = proof_bytes.len();

    metrics
}

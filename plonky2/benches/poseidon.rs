use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::util::serialization::{DefaultGateSerializer, DefaultGeneratorSerializer, Write};
use plonky2_sha256::PLONKY2_BENCH_PROPERTIES;
use plonky2_sha256::bench::{poseidon_prepare, prove, verify};
use utils::harness::ProvingSystem;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

utils::define_benchmark_harness!(
    BenchTarget::Poseidon,
    ProvingSystem::Plonky2,
    None,
    "poseidon_mem_plonky2",
    PLONKY2_BENCH_PROPERTIES,
    poseidon_prepare,
    |(_, _, n_gates)| *n_gates,
    |(circuit_data, pw, _)| { prove(circuit_data, pw.clone()) },
    |(circuit_data, _pw, _), proof| {
        let verifier_data = circuit_data.verifier_data();
        verify(&verifier_data, proof.clone());
    },
    |(circuit_data, _pw, _)| {
        let gate_serializer = DefaultGateSerializer;
        let common_data_size = circuit_data
            .common
            .to_bytes(&gate_serializer)
            .unwrap()
            .len();
        let generator_serializer = DefaultGeneratorSerializer::<C, D>::default();
        let prover_data_size = circuit_data
            .prover_only
            .to_bytes(&generator_serializer, &circuit_data.common)
            .unwrap()
            .len();
        prover_data_size + common_data_size
    },
    |proof| {
        let mut buffer = Vec::new();
        buffer.write_proof(&proof.proof).unwrap();
        buffer.len()
    }
);

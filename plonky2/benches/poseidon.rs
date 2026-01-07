use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::serialization::{DefaultGateSerializer, DefaultGeneratorSerializer, Write};
use plonky2_sha256::bench::{poseidon_prepare, prove, verify};
use utils::harness::{AuditStatus, BenchProperties, ProvingSystem};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

utils::define_benchmark_harness!(
    BenchTarget::Poseidon,
    ProvingSystem::Plonky2,
    None,
    "poseidon_mem_plonky2",
    BenchProperties::new(
        "Plonky2",    // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        "Goldilocks", // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        "FRI",        // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        Some("FRI"),  // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        "Plonkish",   // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        true,         // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        100,          // https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#security
        true,         // hash-based PCS
        false, // deprecated: https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#%EF%B8%8F-plonky2-deprecation-notice
        AuditStatus::Audited, // https://github.com/0xPolygonZero/plonky2/tree/main/audits
        None,
    ),
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

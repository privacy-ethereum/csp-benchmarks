use plonky2_sha256::bench::{prove, sha256_prepare, verify};

use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};
use utils::harness::{AuditStatus, BenchProperties, ProvingSystem};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Plonky2,
    None,
    "sha256_no_lookup_mem",
    BenchProperties {
        proving_system: Some("Plonky2".to_string()), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        field_curve: Some("Goldilocks".to_string()), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        iop: Some("FRI".to_string()), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        pcs: Some("FRI".to_string()), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        arithm: Some("Plonkish".to_string()), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        is_zk: Some(true), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
        security_bits: Some(100), // https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#security
        is_pq: Some(true),        // hash-based PCS
        is_maintained: Some(false), // deprecated: https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#%EF%B8%8F-plonky2-deprecation-notice
        is_audited: Some(AuditStatus::Audited), // https://github.com/0xPolygonZero/plonky2/tree/main/audits
        isa: None,
    },
    sha256_prepare,
    |(circuit_data, _)| { circuit_data.common.num_gate_constraints },
    |(circuit_data, pw)| { prove(circuit_data, pw.clone()) },
    |(circuit_data, _pw), proof| {
        let verifier_data = circuit_data.verifier_data();
        verify(&verifier_data, proof.clone());
    },
    |(circuit_data, _pw)| {
        let gate_serializer = U32GateSerializer;
        let common_data_size = circuit_data
            .common
            .to_bytes(&gate_serializer)
            .unwrap()
            .len();
        let generator_serializer = U32GeneratorSerializer::<C, D>::default();
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

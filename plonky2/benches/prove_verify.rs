use plonky2_sha256::bench::{prove, sha256_prepare, verify};

use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};
use utils::harness::{BenchHarnessConfig, ProvingSystem};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

utils::define_benchmark_harness!(
    sha256,
    BenchHarnessConfig::sha256(ProvingSystem::Plonky2, None, Some("sha256_no_lookup_mem")),
    |input_size| { sha256_prepare(input_size) },
    |(circuit_data, pw)| { prove(&circuit_data, pw.clone()) },
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

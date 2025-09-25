use provekit::{prepare_sha256, preprocessing_size, prove, verify};
use utils::harness::{BenchHarnessConfig, ProvingSystem};

utils::define_benchmark_harness!(
    benches,
    BenchHarnessConfig::sha256(ProvingSystem::Provekit, None, Some("sha256_mem")),
    |input_size| { prepare_sha256(input_size) },
    |(proof_scheme, toml_path, _)| { prove(proof_scheme, toml_path) },
    |(proof_scheme, _, _), proof| {
        verify(&proof, proof_scheme).unwrap();
    },
    |(_, _, circuit_path)| { preprocessing_size(&circuit_path) },
    |proof| { proof.whir_r1cs_proof.transcript.len() }
);

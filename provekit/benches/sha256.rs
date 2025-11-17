use provekit::{PROVEKIT_PROPS, prepare_sha256, preprocessing_size, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Provekit,
    None,
    "sha256_mem_provekit",
    PROVEKIT_PROPS,
    prepare_sha256,
    |(proof_scheme, _, _)| { proof_scheme.r1cs.num_constraints() },
    |(proof_scheme, toml_path, _)| { prove(proof_scheme, toml_path) },
    |(proof_scheme, _, _), proof| {
        verify(proof, proof_scheme).unwrap();
    },
    |(_, _, circuit_path)| { preprocessing_size(circuit_path) },
    |proof| { proof.whir_r1cs_proof.transcript.len() }
);

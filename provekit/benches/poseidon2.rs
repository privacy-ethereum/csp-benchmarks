use provekit::{PROVEKIT_PROPS, prepare_poseidon2, preprocessing_size, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Poseidon2,
    ProvingSystem::Provekit,
    None,
    "poseidon2_mem_provekit",
    PROVEKIT_PROPS,
    prepare_poseidon2,
    |(proof_scheme, _, _)| { proof_scheme.r1cs().num_constraints() },
    |(proof_scheme, toml_path, _)| { prove(proof_scheme, toml_path) },
    |(proof_scheme, _, _), proof| {
        verify(proof, proof_scheme).unwrap();
    },
    |(_, _, circuit_path)| { preprocessing_size(circuit_path) },
    |proof| { proof.whir_r1cs_proof.narg_string.len() + proof.whir_r1cs_proof.hints.len() }
);

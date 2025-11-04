use provekit::{prepare_sha256, preprocessing_size, prove, verify};
use utils::harness::{AuditStatus, BenchProperties, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Provekit,
    None,
    "sha256_mem_provekit",
    BenchProperties::new(
        "Spartan+WHIR", // https://github.com/worldfnd/provekit
        "Bn254",        // https://github.com/worldfnd/provekit
        "Spartan",      // https://github.com/worldfnd/provekit
        Some("WHIR"),   // https://github.com/worldfnd/provekit
        "R1CS",         // https://github.com/worldfnd/provekit
        true,           // https://github.com/worldfnd/provekit/pull/138
        128, // https://github.com/worldfnd/provekit/blob/d7deea66c41d56c1d411dd799d0d6066272323e4/provekit/r1cs-compiler/src/whir_r1cs.rs#L43
        true, // hash-based PCS
        true, // https://github.com/worldfnd/provekit
        AuditStatus::NotAudited,
        None
    ),
    prepare_sha256,
    |(proof_scheme, _, _)| { proof_scheme.r1cs.num_constraints() },
    |(proof_scheme, toml_path, _)| { prove(proof_scheme, toml_path) },
    |(proof_scheme, _, _), proof| {
        verify(proof, proof_scheme).unwrap();
    },
    |(_, _, circuit_path)| { preprocessing_size(circuit_path) },
    |proof| { proof.whir_r1cs_proof.transcript.len() }
);

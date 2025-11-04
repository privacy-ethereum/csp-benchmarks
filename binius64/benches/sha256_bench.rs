use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_utils::serialization::SerializeBytes;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::prepare;
use utils::harness::{AuditStatus, BenchProperties, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Binius64,
    None,
    "sha256_mem_binius64",
    BenchProperties::new(
        "Binius64",
        "GHASH binary field", // https://www.binius.xyz/basics/binius64-vs-v0
        "Binius64",
        Some("Binius64"),
        "Binius64",
        true, // https://www.binius.xyz/basics/binius64-vs-v0
        96, // https://github.com/IrreducibleOSS/binius64/blob/main/verifier/verifier/src/verify.rs#L40
        true, // hash-based PCS
        true,
        AuditStatus::NotAudited,
        None,
    ),
    |input_size| {
        prepare(input_size).expect("Failed to prepare sha256 circuit for prove/verify")
    },
    |(_, _, cs, _, _, _)| { cs.n_and_constraints() + cs.n_mul_constraints() },
    |(_verifier, prover, _cs, sha256_circuit, compiled_circuit, input_size)| {
        binius64::prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            prover,
            compiled_circuit,
            sha256_circuit,
            *input_size,
        )
        .expect("Failed to prove sha256 circuit")
    },
    |(verifier, _prover, _cs, _sha256_circuit, _compiled_circuit, _input_size),
     (proof, pub_witness)| {
        binius64::verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            verifier,
            pub_witness,
            proof,
        )
        .expect("Failed to verify sha256 circuit")
    },
    |(_verifier, _prover, cs, _sha256_circuit, _compiled_circuit, _input_size)| {
        let mut buf: Vec<u8> = Vec::new();
        cs.serialize(&mut buf)
            .expect("Failed to serialize constraint system into byte array");
        buf.len()
    },
    |(proof, _pub_witness)| proof.len()
);

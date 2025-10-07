use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_utils::serialization::SerializeBytes;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::prepare;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Binius64,
    None,
    "sha256_mem_binius64",
    |input_size| {
        prepare(input_size).expect("Failed to prepare sha256 circuit for prove/verify")
    },
    |(_verifier, prover, witness, _cs)| {
        binius64::prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            prover,
            witness.clone(),
        )
        .expect("Failed to prove sha256 circuit")
    },
    |(verifier, _prover, witness, _cs), proof| {
        binius64::verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            verifier,
            witness.clone(),
            proof,
        )
        .expect("Failed to verify sha256 circuit")
    },
    |(_verifier, _prover, _witness, cs)| {
        let mut buf: Vec<u8> = Vec::new();
        cs.serialize(&mut buf)
            .expect("Failed to serialize constraint system into byte array");
        buf.len()
    },
    |proof: &Vec<u8>| proof.len()
);

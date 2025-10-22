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
    |(_verifier, prover, _cs, example, circuit, input_size)| {
        binius64::prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            prover,
            circuit,
            example,
            *input_size,
        )
        .expect("Failed to prove sha256 circuit")
    },
    |(verifier, _prover, _cs, _example, _circuit, _input_size), (proof, pub_witness)| {
        binius64::verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            verifier,
            pub_witness,
            proof,
        )
        .expect("Failed to verify sha256 circuit")
    },
    |(_verifier, _prover, cs, _example, _circuit, _input_size)| {
        let mut buf: Vec<u8> = Vec::new();
        cs.serialize(&mut buf)
            .expect("Failed to serialize constraint system into byte array");
        buf.len()
    },
    |(proof, _pub_witness)| proof.len()
);

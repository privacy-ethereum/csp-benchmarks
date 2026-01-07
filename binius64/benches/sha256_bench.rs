use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::{
    BINIUS64_BENCH_PROPERTIES,
    circuits::{Sha256Circuit, sha256::Sha256Params},
    count_constraints, prepare, preprocessing_size, proof_size,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Binius64,
    None,
    "sha256_mem_binius64",
    BINIUS64_BENCH_PROPERTIES,
    |input_size| {
        prepare::<Sha256Circuit>(
            input_size,
            Sha256Params {
                max_len_bytes: Some(input_size),
                exact_len: true,
            },
        )
        .expect("Failed to prepare sha256 circuit for prove/verify")
    },
    |(_, _, cs, _, _, _)| count_constraints(cs),
    |(_verifier, prover, _cs, sha256_circuit, compiled_circuit, input_size)| {
        binius64::prove::<
            StdDigest,
            StdCompression,
            ParallelCompressionAdaptor<StdCompression>,
            Sha256Circuit,
        >(prover, compiled_circuit, sha256_circuit, *input_size)
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
    |(_, _, cs, _, _, _)| preprocessing_size(cs),
    |proof| proof_size(proof)
);

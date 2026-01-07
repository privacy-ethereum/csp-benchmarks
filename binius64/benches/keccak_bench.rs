use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::{
    BINIUS64_BENCH_PROPERTIES,
    circuits::{KeccakCircuit, keccak::KeccakParams},
    count_constraints, prepare, preprocessing_size, proof_size,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Binius64,
    None,
    "keccak_mem_binius64",
    BINIUS64_BENCH_PROPERTIES,
    |input_size| {
        prepare::<KeccakCircuit>(
            input_size,
            KeccakParams {
                max_len_bytes: Some(input_size),
            },
        )
        .expect("Failed to prepare keccak circuit for prove/verify")
    },
    |(_, _, cs, _, _, _)| count_constraints(cs),
    |(_verifier, prover, _cs, keccak_circuit, compiled_circuit, input_size)| {
        binius64::prove::<
            StdDigest,
            StdCompression,
            ParallelCompressionAdaptor<StdCompression>,
            KeccakCircuit,
        >(prover, compiled_circuit, keccak_circuit, *input_size)
        .expect("Failed to prove keccak circuit")
    },
    |(verifier, _prover, _cs, _keccak_circuit, _compiled_circuit, _input_size),
     (proof, pub_witness)| {
        binius64::verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            verifier,
            pub_witness,
            proof,
        )
        .expect("Failed to verify keccak circuit")
    },
    |(_, _, cs, _, _, _)| preprocessing_size(cs),
    |proof| proof_size(proof)
);

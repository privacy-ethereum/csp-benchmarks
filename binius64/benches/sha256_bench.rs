// Copyright 2024-2025 Irreducible Inc.

use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_utils::serialization::SerializeBytes;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::prepare;
use utils::harness::{BenchHarnessConfig, ProvingSystem};

utils::define_benchmark_harness!(
    sha256,
    BenchHarnessConfig::sha256(ProvingSystem::Binius64, None, Some("sha256_mem")),
    |input_size| {
        prepare(input_size).expect("Failed to prepare sha256 circuit for prove/verify")
    },
    |prepared_context| {
        let (_verifier, prover, witness, _cs) = prepared_context;
        binius64::prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            prover,
            witness.clone(),
        )
        .expect("Failed to prove sha256 circuit")
    },
    |prepared_context, proof| {
        let (verifier, _prover, witness, _cs) = prepared_context;
        binius64::verify::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
            verifier,
            witness.clone(),
            proof,
        )
        .expect("Failed to verify sha256 circuit")
    },
    |prepared_context| {
        let (_verifier, _prover, _witness, cs) = prepared_context;
        let mut buf: Vec<u8> = Vec::new();
        cs.serialize(&mut buf)
            .expect("Failed to serialize constraint system into byte array");
        buf.len()
    },
    |proof: &Vec<u8>| proof.len()
);

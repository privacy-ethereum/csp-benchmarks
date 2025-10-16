use cairo_m::{prepare, prove, verify};
use cairo_m_prover::Proof;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    |input_size| { prepare(input_size) },
    |runner_output| { prove(runner_output) },
    |_runner_output, proof| { verify(proof) },
    |_runner_output| { 0 },
    |proof| proof.stark_proof.size_estimate()
);

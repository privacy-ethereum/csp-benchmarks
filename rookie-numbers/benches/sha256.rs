//! SHA256 benchmark using Rookie Numbers prover.

use rookie_numbers::{secure_pcs_config, ROOKIE_NUMBERS_BENCH_PROPERTIES};
use sha256::{prove_sha256, verify_sha256};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::RookieNumbers,
    None,
    "sha256_mem_rookie_numbers",
    ROOKIE_NUMBERS_BENCH_PROPERTIES,
    // prepare: |input_size| -> PreparedContext
    |input_size| utils::generate_sha256_input(input_size).0,
    // num_constraints: |ctx| -> usize
    |_words| 0,
    // prove: |words| -> Proof
    |words| prove_sha256(&words, secure_pcs_config()),
    // verify: |words, proof| -> ()
    |_words, proof| verify_sha256(proof.0.clone(), proof.1, &proof.2).expect("verify failed"),
    // preprocessing_size: |words| -> usize
    |_words| 0,
    // proof_size: |proof| -> usize
    |proof| bincode::serialize(proof).map(|v| v.len()).unwrap_or(0)
);

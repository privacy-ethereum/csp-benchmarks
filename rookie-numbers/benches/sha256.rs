//! SHA256 benchmark using Rookie Numbers prover.

use rookie_numbers::{
    num_constraints, prepare, preprocessing_size, proof_size, prove, verify,
    ROOKIE_NUMBERS_BENCH_PROPERTIES,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::RookieNumbers,
    None,
    "sha256_mem_rookie_numbers",
    ROOKIE_NUMBERS_BENCH_PROPERTIES,
    // prepare: |input_size| -> PreparedContext
    |input_size| prepare(input_size),
    // num_constraints: |ctx| -> usize
    |ctx| num_constraints(ctx),
    // prove: |ctx| -> Proof
    |ctx| prove(ctx),
    // verify: |ctx, proof| -> ()
    |ctx, proof| verify(ctx, proof),
    // preprocessing_size: |ctx| -> usize
    |ctx| preprocessing_size(ctx),
    // proof_size: |proof| -> usize
    |proof| proof_size(proof)
);

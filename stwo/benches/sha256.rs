//! SHA256 benchmark using Stwo prover.

use stwo_bench::{
    num_constraints, prepare, preprocessing_size, proof_size, prove, verify, STWO_BENCH_PROPERTIES,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Stwo,
    None,
    "sha256_mem_stwo",
    STWO_BENCH_PROPERTIES,
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

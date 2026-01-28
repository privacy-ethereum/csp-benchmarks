//! Rookie Numbers SHA256 benchmark integration for csp-benchmarks.
//!
//! This crate wraps the Rookie Numbers SHA256 prover from rookie-numbers to integrate
//! with the csp-benchmarks Rust harness.

use std::borrow::Cow;

use sha256::components::ClaimedSum;
use sha256::preprocessed::PreProcessedTrace;
use sha256::{
    prove_sha256, verify_sha256, Blake2sMerkleHasher, Column, FriConfig, PcsConfig, StarkProof,
};
use utils::harness::{AuditStatus, BenchProperties};

/// Benchmark properties for Rookie Numbers prover (using Stwo library).
pub const ROOKIE_NUMBERS_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Rookie Numbers"),
    field_curve: Cow::Borrowed("M31"),
    iop: Cow::Borrowed("FRI"),
    pcs: Some(Cow::Borrowed("Circle-PCS")),
    arithm: Cow::Borrowed("AIR"),
    is_zk: false,
    is_zkvm: false,
    security_bits: 96,
    is_pq: true,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

/// Proof type returned by Stwo prover.
/// Contains both the STARK proof and the claimed sums needed for verification.
pub type StwoProof = (StarkProof<Blake2sMerkleHasher>, ClaimedSum);

/// Prepared context for SHA256 benchmarks.
pub struct PreparedSha256 {
    /// Log2 of the number of SHA256 instances
    pub log_size: u32,
    /// PCS configuration
    pub config: PcsConfig,
    /// Original input size in bytes (for reference)
    pub input_size: usize,
}

/// Convert input size in bytes to log_size.
///
/// Rookie Numbers processes SHA256 in batches. The log_size is the log2 of the number
/// of SHA256 compression function instances. For benchmarking, we map:
/// - Small inputs (128-2048 bytes) to reasonable log_sizes that demonstrate
///   the prover's performance.
///
/// This mapping ensures we're benchmarking actual SHA256 proving work,
/// not just setup overhead.
fn input_size_to_log_size(input_size: usize) -> u32 {
    // SHA256 block size is 64 bytes (512 bits)
    // Each SHA256 instance processes one block
    // For input_size bytes, we need ceil(input_size / 64) blocks
    // But Stwo's log_size represents 2^log_size instances
    //
    // We use a mapping that provides meaningful benchmark sizes:
    // 128 bytes  -> log_size 8  (256 instances)
    // 256 bytes  -> log_size 9  (512 instances)
    // 512 bytes  -> log_size 10 (1024 instances)
    // 1024 bytes -> log_size 11 (2048 instances)
    // 2048 bytes -> log_size 12 (4096 instances)
    //
    // This gives us a good range of proving work while keeping times reasonable.
    match input_size {
        0..=128 => 8,
        129..=256 => 9,
        257..=512 => 10,
        513..=1024 => 11,
        1025..=2048 => 12,
        2049..=4096 => 13,
        4097..=8192 => 14,
        _ => 15, // Cap at log_size 15 for very large inputs
    }
}

fn secure_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 70,
        },
    }
}
/// Prepare the Rookie Numbers prover for benchmarking.
///
/// # Arguments
/// * `input_size` - Input size in bytes (from the harness)
///
/// # Returns
/// Prepared context containing log_size and PCS configuration.
pub fn prepare(input_size: usize) -> PreparedSha256 {
    let log_size = input_size_to_log_size(input_size);

    PreparedSha256 {
        log_size,
        config: secure_pcs_config(),
        input_size,
    }
}

/// Generate a STARK proof for SHA256.
///
/// # Arguments
/// * `ctx` - Prepared benchmark context
///
/// # Returns
/// Tuple of (StarkProof, ClaimedSum) needed for verification.
pub fn prove(ctx: &PreparedSha256) -> StwoProof {
    prove_sha256(ctx.log_size, ctx.config)
}

/// Verify a SHA256 STARK proof.
///
/// # Arguments
/// * `ctx` - Prepared benchmark context
/// * `proof` - The proof to verify (includes claimed sums)
pub fn verify(ctx: &PreparedSha256, proof: &StwoProof) {
    let (stark_proof, claimed_sum) = proof;
    verify_sha256(stark_proof.clone(), ctx.log_size, claimed_sum).expect("Verification failed");
}

/// Get the number of constraints (trace length) for the proof.
///
/// For Rookie Numbers AIR, this is the trace length: 64 rounds × 2^log_size instances.
///
/// # Arguments
/// * `ctx` - Prepared benchmark context
///
/// # Returns
/// Total number of trace rows (constraint count equivalent for AIR).
pub fn num_constraints(ctx: &PreparedSha256) -> usize {
    // Rookie Numbers SHA256 trace: 64 rounds per instance × number of instances
    64 * (1 << ctx.log_size)
}

/// Get the preprocessing size in bytes.
///
/// This measures the size of the preprocessed trace columns.
///
/// # Arguments
/// * `ctx` - Prepared benchmark context
///
/// # Returns
/// Size in bytes of the preprocessed data.
pub fn preprocessing_size(ctx: &PreparedSha256) -> usize {
    let preprocessed_trace = PreProcessedTrace::new(ctx.log_size);

    // Sum up sizes of all preprocessed column evaluations
    // Each CircleEvaluation contains values that are M31 field elements (4 bytes each)
    preprocessed_trace
        .trace
        .iter()
        .map(|eval| {
            // values.len() gives the number of field elements
            // Each M31 element is 4 bytes
            eval.values.len() * 4
        })
        .sum()
}

/// Get the proof size in bytes.
///
/// # Arguments
/// * `proof` - The generated proof
///
/// # Returns
/// Size in bytes of the serialized proof.
pub fn proof_size(proof: &StwoProof) -> usize {
    // Serialize the proof using bincode
    // Note: Only serialize the StarkProof, not the ClaimedSum (which is internal state)
    bincode::serialize(&proof.0).map(|v| v.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_size_mapping() {
        assert_eq!(input_size_to_log_size(128), 8);
        assert_eq!(input_size_to_log_size(256), 9);
        assert_eq!(input_size_to_log_size(512), 10);
        assert_eq!(input_size_to_log_size(1024), 11);
        assert_eq!(input_size_to_log_size(2048), 12);
    }

    #[test]
    fn test_prepare() {
        let ctx = prepare(128);
        assert_eq!(ctx.log_size, 8);
        assert_eq!(ctx.input_size, 128);
    }

    #[test]
    fn test_num_constraints() {
        let ctx = prepare(128);
        // log_size=8 means 2^8=256 instances, each with 64 rounds
        assert_eq!(num_constraints(&ctx), 64 * 256);
    }
}

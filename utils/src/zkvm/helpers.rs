use crate::zkvm::{PreparedSha256, ProofArtifacts};
use std::path::PathBuf;
use zkvm_interface::zkVM;

/// Prove a SHA-256 benchmark using the prepared zkVM instance.
pub fn prove_sha256<V: zkVM>(prepared: &PreparedSha256<V>) -> ProofArtifacts {
    prepared.prove().expect("prove failed")
}

/// Verify a SHA-256 proof with digest checking.
pub fn verify_sha256<V: zkVM>(prepared: &PreparedSha256<V>, proof: &ProofArtifacts) {
    prepared.verify_with_digest(proof).expect("verify failed");
}

/// Get the execution cycles for the prepared program.
pub fn execution_cycles<V: zkVM>(prepared: &PreparedSha256<V>) -> u64 {
    prepared.execution_cycles().expect("execute failed")
}

/// Get the preprocessing (compiled program) size.
pub fn preprocessing_size<V>(prepared: &PreparedSha256<V>) -> usize {
    prepared.compiled_size()
}

/// Get the proof size from proof artifacts.
pub fn proof_size(proof: &ProofArtifacts) -> usize {
    proof.proof_size()
}

/// Get the guest program directory path for a benchmark.
pub fn guest_dir(benchmark_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest")
        .join(benchmark_name)
}

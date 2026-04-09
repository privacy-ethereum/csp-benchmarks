mod ffi;

use std::ffi::c_void;
use std::path::PathBuf;
use utils::harness::{AuditStatus, BenchProperties};

// ---------------------------------------------------------------------------
// Safe wrappers around the Zig C-API opaque handles
// ---------------------------------------------------------------------------

/// Shared state across all benchmark iterations. Holds the Zig thread pool.
/// Implements `Copy` (required by the harness) — it's just a raw pointer.
/// The thread pool is intentionally leaked (lives for the entire process).
#[derive(Clone, Copy)]
pub struct ZoltSharedState {
    tp_handle: *mut c_void,
}

// SAFETY: The thread pool is accessed only through the C API which
// internally synchronises via its work-stealing queues.
unsafe impl Send for ZoltSharedState {}
unsafe impl Sync for ZoltSharedState {}

impl ZoltSharedState {
    pub fn new() -> Self {
        let tp_handle = unsafe { ffi::zolt_thread_pool_create() };
        assert!(!tp_handle.is_null(), "zolt: failed to create thread pool");
        Self { tp_handle }
    }
}

/// Prepared context for a single SHA-256 input size.
/// Holds the loaded ELF and a reference to the shared thread pool.
pub struct PreparedZoltSha256 {
    elf_handle: *mut c_void,
    elf_byte_size: usize,
}

impl Drop for PreparedZoltSha256 {
    fn drop(&mut self) {
        if !self.elf_handle.is_null() {
            unsafe { ffi::zolt_loaded_elf_destroy(self.elf_handle) };
        }
    }
}

/// Proof artifact returned by `prove_sha256`.
pub struct ZoltProofResult {
    handle: *mut c_void,
}

impl Drop for ZoltProofResult {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::zolt_proof_result_destroy(self.handle) };
        }
    }
}

// ---------------------------------------------------------------------------
// Benchmark harness functions (matching Jolt's pattern exactly)
// ---------------------------------------------------------------------------

pub fn zolt_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "Zolt",
        "Bn254",
        "Twist & Shout",
        Some("Dory"),
        "Jolt",
        false, // is_zk
        true,  // is_zkvm
        128,   // security_bits
        false, // is_pq
        true,  // is_maintained
        AuditStatus::NotAudited,
        Some("RISC-V RV64IMC"),
    )
}

/// Maps input_size (bytes) to the corresponding pre-built ELF path.
fn elf_path_for_size(input_size: usize) -> PathBuf {
    let examples_dir = PathBuf::from(env!("ZOLT_EXAMPLES_DIR"));
    if input_size == 256 {
        examples_dir.join("sha256.elf")
    } else {
        examples_dir.join(format!("sha256_{input_size}.elf"))
    }
}

/// Prepare: load ELF + init prover context. NOT timed by Criterion.
pub fn prepare_sha256(input_size: usize, shared: ZoltSharedState) -> PreparedZoltSha256 {
    let path = elf_path_for_size(input_size);
    let path_bytes = path.to_str().expect("non-UTF8 path").as_bytes();

    let elf_handle = unsafe {
        ffi::zolt_load_elf(shared.tp_handle, path_bytes.as_ptr(), path_bytes.len())
    };
    assert!(
        !elf_handle.is_null(),
        "zolt: failed to load ELF at {}",
        path.display()
    );

    let elf_byte_size = unsafe { ffi::zolt_loaded_elf_size(elf_handle) };

    PreparedZoltSha256 {
        elf_handle,
        elf_byte_size,
    }
}

/// Prove: full pipeline (trace + preprocess + prove). TIMED by Criterion.
pub fn prove_sha256(prepared: &PreparedZoltSha256, _shared: &ZoltSharedState) -> ZoltProofResult {
    let handle = unsafe { ffi::zolt_prove(prepared.elf_handle) };
    assert!(!handle.is_null(), "zolt: proving failed");
    ZoltProofResult { handle }
}

/// Verify: not yet implemented — no-op.
pub fn verify_sha256(
    _prepared: &PreparedZoltSha256,
    _proof: &ZoltProofResult,
    _shared: &ZoltSharedState,
) {
}

/// Returns ELF bytecode size as a proxy for preprocessing size.
pub fn preprocessing_size(prepared: &PreparedZoltSha256, _shared: &ZoltSharedState) -> usize {
    prepared.elf_byte_size
}

/// Returns serialized proof size in bytes.
pub fn proof_size(proof: &ZoltProofResult, _shared: &ZoltSharedState) -> usize {
    unsafe { ffi::zolt_proof_result_size(proof.handle) }
}

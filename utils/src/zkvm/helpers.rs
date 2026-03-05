use crate::zkvm::ecdsa::PreparedEcdsa;
use crate::zkvm::hash::PreparedHash;
use crate::zkvm::instance::{CompiledProgram, ProofArtifacts, compile_guest_program};
use crate::zkvm::traits::PreparedBenchmark;
use bincode::Options;
use ere_zkvm_interface::Compiler;
use ere_zkvm_interface::zkVM;
use std::any::type_name;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::SystemTime;

/// Prove any benchmark using the prepared zkVM instance.
pub fn prove<P: PreparedBenchmark, SharedState>(prepared: &P, _: &SharedState) -> ProofArtifacts {
    prepared.prove().expect("prove failed")
}

/// Prove a SHA-256 benchmark
pub use prove as prove_sha256;

/// Prove an ECDSA benchmark (type-specific wrapper for compatibility).
pub fn prove_ecdsa<V: zkVM, SharedState>(
    prepared: &PreparedEcdsa<V>,
    shared_state: &SharedState,
) -> ProofArtifacts {
    prove(prepared, shared_state)
}

/// Verify a hash proof with digest checking.
pub fn verify_hash<V: zkVM, SharedState>(
    prepared: &PreparedHash<V>,
    proof: &ProofArtifacts,
    _: &SharedState,
) {
    prepared.verify_with_digest(proof).expect("verify failed");
}

/// Verify a SHA-256 proof with digest checking.
pub use verify_hash as verify_sha256;

/// Verify a Keccak proof with digest checking.
pub use verify_hash as verify_keccak;

/// Verify an ECDSA proof with expected values checking.
pub fn verify_ecdsa<V: zkVM, SharedState>(
    prepared: &PreparedEcdsa<V>,
    proof: &ProofArtifacts,
    _: &SharedState,
) {
    prepared.verify_with_expected(proof).expect("verify failed");
}

/// Get the execution cycles for any prepared benchmark.
pub fn execution_cycles<P: PreparedBenchmark>(prepared: &P) -> u64 {
    prepared.execution_cycles().expect("execute failed")
}

/// Get the preprocessing (compiled program) size for any prepared benchmark.
pub fn preprocessing_size<P: PreparedBenchmark, SharedState>(
    prepared: &P,
    _: &SharedState,
) -> usize {
    prepared.compiled_size()
}

/// Get the proof size from proof artifacts.
pub fn proof_size<SharedState>(proof: &ProofArtifacts, _: &SharedState) -> usize {
    proof.proof_size()
}

/// Get the guest program directory path for a benchmark.
pub fn guest_dir(benchmark_name: &str) -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .join("guest")
        .join(benchmark_name)
}

/// Compute the standard compiled program path for a benchmark.
/// By convention we store at guest/<bench>/target/<bench>_<compiler>.bin
pub fn compiled_program_path<C: Compiler>(benchmark_name: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    type_name::<C>().hash(&mut hasher);
    let compiler_key = format!("{:x}", hasher.finish());

    guest_dir(benchmark_name)
        .join("target")
        .join(format!("{}_{}.bin", benchmark_name, compiler_key))
}

fn newest_guest_source_mtime(path: &std::path::Path) -> Option<SystemTime> {
    let mut newest = None;
    let entries = fs::read_dir(path).ok()?;

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let file_name = entry.file_name();

        if file_name == "target" {
            continue;
        }

        if entry_path.is_dir() {
            if let Some(child_newest) = newest_guest_source_mtime(&entry_path) {
                newest = Some(newest.map_or(child_newest, |t: SystemTime| t.max(child_newest)));
            }
            continue;
        }

        // Keep this as nested `if let` instead of let-chains (`if let ... && let ...`):
        // Nexus builds with nightly-2025-04-06, where let-chains in this position are
        // still unstable and fail CI with E0658.
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                newest = Some(newest.map_or(modified, |t: SystemTime| t.max(modified)));
            }
        }
    }

    newest
}

fn is_compiled_program_stale(compiled_path: &std::path::Path, benchmark_name: &str) -> bool {
    let compiled_mtime = compiled_path.metadata().and_then(|m| m.modified()).ok();
    let guest_mtime = newest_guest_source_mtime(&guest_dir(benchmark_name));

    match (compiled_mtime, guest_mtime) {
        (Some(compiled), Some(guest)) => guest > compiled,
        (_, Some(_)) => true,
        _ => false,
    }
}

/// Load a compiled program, panicking if it is missing.
/// Used by RAM measurement binaries which must never trigger compilation.
pub fn load_compiled_program<C: Compiler>(benchmark_name: &str) -> CompiledProgram<C> {
    let compiled_path = compiled_program_path::<C>(benchmark_name);
    let program_bin = fs::read(&compiled_path)
        .expect("missing compiled guest; the harness should have compiled it already");
    let program: C::Program = bincode::options()
        .deserialize(&program_bin)
        .expect("failed to deserialize compiled program");
    let byte_size = program_bin.len();
    CompiledProgram { program, byte_size }
}

/// Load a compiled program if present, otherwise compile and persist it.
///
/// The cwd is saved and restored around compilation because some compiler
/// implementations (notably Jolt's `RustRv64imacCustomized`) call
/// `std::env::set_current_dir` and never restore it.
pub fn load_or_compile_program<C: Compiler>(
    compiler: &C,
    benchmark_name: &str,
) -> CompiledProgram<C> {
    let compiled_path = compiled_program_path::<C>(benchmark_name);
    if compiled_path.exists() && !is_compiled_program_stale(&compiled_path, benchmark_name) {
        load_compiled_program(benchmark_name)
    } else {
        let original_dir =
            std::env::current_dir().expect("failed to get current working directory");

        let program = compile_guest_program(compiler, &guest_dir(benchmark_name))
            .expect("failed to compile guest program");

        // Restore cwd in case the compiler changed it.
        std::env::set_current_dir(&original_dir).expect("failed to restore working directory");

        let bytes = bincode::options()
            .serialize(&program.program)
            .expect("failed to serialize compiled program");
        fs::create_dir_all(compiled_path.parent().unwrap()).expect("failed to create directory");
        fs::write(&compiled_path, bytes).expect("failed to write compiled program file");
        program
    }
}

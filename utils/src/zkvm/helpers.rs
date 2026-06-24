use crate::zkvm::ecdsa::PreparedEcdsa;
use crate::zkvm::hash::PreparedHash;
use crate::zkvm::instance::{CompiledProgram, ProofArtifacts, compile_guest_program};
use crate::zkvm::private_tx::PreparedPrivateTx;
use crate::zkvm::traits::PreparedBenchmark;
use bincode::Options;
use ere_zkvm_interface::Compiler;
use ere_zkvm_interface::zkVM;
use std::fs;
use std::path::PathBuf;

/// Prove any benchmark using the prepared zkVM instance.
pub fn prove<P: PreparedBenchmark, SharedState>(prepared: &P, _: &SharedState) -> ProofArtifacts {
    prepared.prove().expect("prove failed")
}

/// Prove a targeted benchmark.
pub use prove as prove_targeted;

/// Prove a SHA-256 benchmark
pub use prove as prove_sha256;

/// Prove a private transaction benchmark.
pub use prove as prove_private_tx;

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

/// Verify a targeted proof with digest checking.
pub use verify_hash as verify_targeted;

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

/// Verify a private transaction proof with public output checking.
pub fn verify_private_tx<V: zkVM, SharedState>(
    prepared: &PreparedPrivateTx<V>,
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
/// By convention we store at guest/<bench>/target/<bench>.bin
pub fn compiled_program_path(benchmark_name: &str) -> PathBuf {
    guest_dir(benchmark_name)
        .join("target")
        .join(format!("{}.bin", benchmark_name))
}

/// Load a compiled program from an explicit file path.
/// Use this in contexts where CARGO_MANIFEST_DIR is unavailable (e.g. mobile/embedded).
pub fn load_compiled_program_from_path<C: Compiler>(path: &std::path::Path) -> CompiledProgram<C> {
    let program_bin = fs::read(path)
        .unwrap_or_else(|e| panic!("failed to read compiled program at {:?}: {}", path, e));
    let program: C::Program = bincode::options()
        .deserialize(&program_bin)
        .expect("failed to deserialize compiled program");
    let byte_size = program_bin.len();
    CompiledProgram { program, byte_size }
}

/// Load a compiled program, panicking if it is missing.
/// Used by RAM measurement binaries which must never trigger compilation.
pub fn load_compiled_program<C: Compiler>(benchmark_name: &str) -> CompiledProgram<C> {
    let compiled_path = compiled_program_path(benchmark_name);
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
    let compiled_path = compiled_program_path(benchmark_name);
    if compiled_path.exists() {
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

use crate::zkvm::instance::{CompiledProgram, compile_guest_program};
use crate::zkvm::{PreparedSha256, ProofArtifacts};
use bincode::Options;
use ere_zkvm_interface::Compiler;
use ere_zkvm_interface::zkVM;
use std::fs;
use std::path::PathBuf;

/// Prove a SHA-256 benchmark using the prepared zkVM instance.
pub fn prove_sha256<V: zkVM, SharedState>(
    prepared: &PreparedSha256<V>,
    _: &SharedState,
) -> ProofArtifacts {
    prepared.prove().expect("prove failed")
}

/// Verify a SHA-256 proof with digest checking.
pub fn verify_sha256<V: zkVM, SharedState>(
    prepared: &PreparedSha256<V>,
    proof: &ProofArtifacts,
    _: &SharedState,
) {
    prepared.verify_with_digest(proof).expect("verify failed");
}

/// Get the execution cycles for the prepared program.
pub fn execution_cycles<V: zkVM>(prepared: &PreparedSha256<V>) -> u64 {
    prepared.execution_cycles().expect("execute failed")
}

/// Get the preprocessing (compiled program) size.
pub fn preprocessing_size<V, SharedState>(prepared: &PreparedSha256<V>, _: &SharedState) -> usize {
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

/// Load a compiled program, panicking if it is missing.
/// Used by RAM measurement binaries which must never trigger compilation.
pub fn load_compiled_program<C: Compiler>(benchmark_name: &str) -> CompiledProgram<C> {
    let compiled_path = compiled_program_path(benchmark_name);
    let program_bin = fs::read(&compiled_path)
        .expect("missing compiled guest; the harness should have compiled it already");
    let program: C::Program = bincode::options()
        .deserialize(&program_bin)
        .expect("failed to deserialize compiled guest program");
    let byte_size = program_bin.len();
    CompiledProgram { program, byte_size }
}

/// Load a compiled program if present, otherwise compile and persist it.
pub fn load_or_compile_program<C: Compiler>(
    compiler: &C,
    benchmark_name: &str,
) -> CompiledProgram<C> {
    let compiled_path = compiled_program_path(benchmark_name);
    if compiled_path.exists() {
        load_compiled_program(benchmark_name)
    } else {
        let program = compile_guest_program(compiler, &guest_dir(benchmark_name))
            .expect("failed to compile guest program");
        let bytes = bincode::options()
            .serialize(&program.program)
            .expect("failed to serialize compiled program");
        fs::create_dir_all(compiled_path.parent().unwrap()).expect("failed to create directory");
        fs::write(&compiled_path, bytes).expect("failed to write compiled program file");
        program
    }
}

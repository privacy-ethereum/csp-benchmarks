//! Stark-V benchmark integration for csp-benchmarks.

use bincode::Options;
use ere_zkvm_interface::{zkVM, Compiler, Input, Proof, ProofKind};
use stark_v_sdk::{StarkV, StarkVCompiler, StarkVProgram};
use std::fs;
use std::path::PathBuf;
use utils::harness::{AuditStatus, BenchProperties};

pub struct CompiledProgram {
    pub program: StarkVProgram,
    pub byte_size: usize,
}

pub struct PreparedBench {
    vm: StarkV,
    input: Input,
    compiled_size: usize,
    expected_digest: Vec<u8>,
}

pub struct ProofResult {
    pub public_values: Vec<u8>,
    pub proof: Proof,
}

pub fn stark_v_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "STARK",
        "M31",
        "STARK",
        Some("FRI"),
        "AIR",
        false, // Not ZK
        true,
        96,
        true,
        true,
        AuditStatus::NotAudited,
        Some("RISC-V RV32IM"),
    )
}

fn guest_dir(bench_name: &str) -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir).join("guest").join(bench_name)
}

fn compiled_path(bench_name: &str) -> PathBuf {
    guest_dir(bench_name)
        .join("target")
        .join(format!("{}.bin", bench_name))
}

pub fn load_or_compile(bench_name: &str) -> CompiledProgram {
    let path = compiled_path(bench_name);
    if path.exists() {
        return load_compiled(bench_name);
    }
    let compiler = StarkVCompiler::new();
    let program = compiler
        .compile(&guest_dir(bench_name))
        .expect("failed to compile guest program");
    let bytes = bincode::options()
        .serialize(&program)
        .expect("failed to serialize compiled program");
    fs::create_dir_all(path.parent().unwrap()).expect("failed to create directory");
    fs::write(&path, &bytes).expect("failed to write compiled program");
    CompiledProgram {
        program,
        byte_size: bytes.len(),
    }
}

pub fn load_compiled(bench_name: &str) -> CompiledProgram {
    let bytes = fs::read(compiled_path(bench_name))
        .expect("missing compiled guest; the harness should have compiled it already");
    let program: StarkVProgram = bincode::options()
        .deserialize(&bytes)
        .expect("failed to deserialize compiled program");
    CompiledProgram {
        program,
        byte_size: bytes.len(),
    }
}

pub fn prepare_sha256(input_size: usize, program: &CompiledProgram) -> PreparedBench {
    let vm = StarkV::new(program.program.clone());
    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    PreparedBench {
        vm,
        input: Input::new().with_prefixed_stdin(message_bytes),
        compiled_size: program.byte_size,
        expected_digest: digest,
    }
}

pub fn prepare_keccak(input_size: usize, program: &CompiledProgram) -> PreparedBench {
    let vm = StarkV::new(program.program.clone());
    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    PreparedBench {
        vm,
        input: Input::new().with_prefixed_stdin(message_bytes),
        compiled_size: program.byte_size,
        expected_digest: digest,
    }
}

pub fn prove_bench(prepared: &PreparedBench, _: &CompiledProgram) -> ProofResult {
    let (public_values, proof, _) = prepared
        .vm
        .prove(&prepared.input, ProofKind::Compressed)
        .expect("prove failed");
    ProofResult {
        public_values,
        proof,
    }
}

pub fn verify_bench(prepared: &PreparedBench, proof: &ProofResult, _: &CompiledProgram) {
    prepared.vm.verify(&proof.proof).expect("verify failed");
    assert_eq!(
        proof.public_values, prepared.expected_digest,
        "public values do not match expected digest"
    );
}

pub fn preprocessing_size(prepared: &PreparedBench, _: &CompiledProgram) -> usize {
    prepared.compiled_size
}

pub fn proof_size(proof: &ProofResult, _: &CompiledProgram) -> usize {
    match &proof.proof {
        Proof::Compressed(bytes) | Proof::Groth16(bytes) => bytes.len(),
    }
}

pub fn execution_cycles(prepared: &PreparedBench) -> u64 {
    let (_, report) = prepared
        .vm
        .execute(&prepared.input)
        .expect("execute failed");
    report.total_num_cycles
}

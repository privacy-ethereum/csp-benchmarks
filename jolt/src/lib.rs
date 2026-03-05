use ere_jolt::{EreJolt, compiler::RustRv64imacCustomized};
use ere_zkvm_interface::compiler::Compiler;
use ere_zkvm_interface::{Input, ProverResource};
use serde::Serialize;
use std::env;
use std::path::Path;
use utils::harness::{AuditStatus, BenchProperties};
use utils::zkvm::{CompiledProgram, PreparedEcdsa, PreparedKeccak, PreparedSha256};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove, prove_ecdsa, prove_sha256,
    verify_ecdsa, verify_keccak, verify_sha256,
};

// FIXME remove if/when the toolchain conflict is resolved.
/// Wrapper around [`RustRv64imacCustomized`] that temporarily removes the
/// `RUSTUP_TOOLCHAIN` environment variable before compiling.
///
/// This workspace pins a nightly toolchain via `rust-toolchain.toml`.  When
/// `cargo bench` (or any cargo subcommand) runs, rustup propagates
/// `RUSTUP_TOOLCHAIN=<nightly>` to every child process.  That overrides the
/// guest-level `rust-toolchain.toml` which pins the stable channel expected by
/// Jolt's `jolt build` CLI (it uses `RUSTC_BOOTSTRAP=1` on stable to unlock
/// nightly features, which doesn't work on a real nightly).
///
/// The workaround lives here (in csp-benchmarks) rather than in `ere` because
/// the toolchain conflict is specific to this workspace's nightly pin.
pub struct JoltCompiler;

impl Compiler for JoltCompiler {
    type Error = <RustRv64imacCustomized as Compiler>::Error;
    type Program = <RustRv64imacCustomized as Compiler>::Program;

    fn compile(&self, guest_directory: &Path) -> Result<Self::Program, Self::Error> {
        // SAFETY: guest compilation is single-threaded and sequential; the
        // variable is restored immediately after the build completes.
        let saved = env::var("RUSTUP_TOOLCHAIN").ok();
        unsafe { env::remove_var("RUSTUP_TOOLCHAIN") };

        let result = RustRv64imacCustomized.compile(guest_directory);

        if let Some(tc) = saved {
            unsafe { env::set_var("RUSTUP_TOOLCHAIN", tc) };
        }

        result
    }
}

pub fn jolt_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "Jolt",
        "Bn254",
        "Twist & Shout",
        Some("Dory"),
        "Jolt",
        true,
        true,
        128,
        false,
        true,
        AuditStatus::NotAudited,
        Some("RISC-V RV64IMAC"),
    )
}

fn build_framed_input(data: Vec<u8>) -> Input {
    let len = data.len() as u32;
    let mut framed = Vec::with_capacity(4 + data.len());
    framed.extend_from_slice(&len.to_le_bytes());
    framed.extend(data);
    Input::new().with_stdin(framed)
}

pub fn prepare_sha256(
    input_size: usize,
    program: &CompiledProgram<JoltCompiler>,
) -> PreparedSha256<EreJolt> {
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_keccak(
    input_size: usize,
    program: &CompiledProgram<JoltCompiler>,
) -> PreparedKeccak<EreJolt> {
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedKeccak::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_ecdsa(
    _input_size: usize,
    program: &CompiledProgram<JoltCompiler>,
) -> PreparedEcdsa<EreJolt> {
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (digest, (pub_key_x, pub_key_y), signature) = utils::generate_ecdsa_k256_input();
    let input = build_ecdsa_jolt_input(&digest, &pub_key_x, &pub_key_y, &signature);

    PreparedEcdsa::new(vm, input, program.byte_size)
}

#[derive(Serialize)]
struct EcdsaInput {
    z: [u64; 4],
    r: [u64; 4],
    s: [u64; 4],
    q: [u64; 8],
}

fn bytes_be_to_u64_4(bytes: &[u8]) -> [u64; 4] {
    let mut result = [0u64; 4];

    for i in 0..4 {
        result[3 - i] = u64::from_be_bytes(bytes[i * 8..(i + 1) * 8].try_into().unwrap());
    }

    result
}

fn bytes_be_to_u64_8(x: &[u8], y: &[u8]) -> [u64; 8] {
    let mut result = [0u64; 8];

    for i in 0..4 {
        result[3 - i] = u64::from_be_bytes(x[i * 8..(i + 1) * 8].try_into().unwrap());
    }
    for i in 0..4 {
        result[7 - i] = u64::from_be_bytes(y[i * 8..(i + 1) * 8].try_into().unwrap());
    }

    result
}

fn build_ecdsa_jolt_input(
    digest: &[u8],
    pub_key_x: &[u8],
    pub_key_y: &[u8],
    signature: &[u8],
) -> Input {
    let ecdsa_input = EcdsaInput {
        z: bytes_be_to_u64_4(digest),
        r: bytes_be_to_u64_4(&signature[..32]),
        s: bytes_be_to_u64_4(&signature[32..]),
        q: bytes_be_to_u64_8(pub_key_x, pub_key_y),
    };

    let serialized = postcard::to_allocvec(&ecdsa_input).expect("failed to serialize ECDSA input");
    build_framed_input(serialized)
}

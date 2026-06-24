use ere_jolt::{EreJolt, compiler::RustRv64imacCustomized};
use ere_zkvm_interface::{Input, ProverResource};
use serde::Serialize;
use std::env;
use utils::harness::{AuditStatus, BenchProperties};
use utils::targeted::ByteHashKind;
use utils::zkvm::{
    CompiledProgram, PreparedEcdsa, PreparedKeccak, PreparedPrivateTx, PreparedSha256,
    PreparedTargeted,
};

// SAFETY: called once before single-threaded JoltSdk construction
fn set_jolt_config(max_trace_length: u64, stack_size: u64, heap_size: u64) {
    unsafe {
        env::set_var("JOLT_MAX_TRACE_LENGTH", max_trace_length.to_string());
        env::set_var("JOLT_STACK_SIZE", stack_size.to_string());
        env::set_var("JOLT_HEAP_SIZE", heap_size.to_string());
        env::set_var("JOLT_MAX_INPUT_SIZE", "4096");
        env::set_var("JOLT_MAX_OUTPUT_SIZE", "4096");
    }
}

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove, prove_ecdsa, prove_private_tx,
    prove_sha256, verify_ecdsa, verify_keccak, verify_private_tx, verify_sha256,
};

pub fn jolt_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "Jolt",
        "Bn254",
        "Twist & Shout",
        Some("Dory"),
        "Jolt",
        true,
        true,
        100, // BN254 pairing security is about 100 bits after exTNFS estimates, see https://eips.ethereum.org/assets/eip-3068/2017-334.pdf and https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-pairing-friendly-curves-12
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
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedSha256<EreJolt> {
    let max_trace_length = if input_size > 1024 { 131072 } else { 65536 };
    set_jolt_config(max_trace_length, 4096, 32768);
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, digest) = utils::generate_sha256_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedSha256::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_keccak(
    input_size: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedKeccak<EreJolt> {
    set_jolt_config(65536, 4096, 32768);
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (message_bytes, digest) = utils::generate_keccak_input(input_size);
    let input = build_framed_input(message_bytes);

    PreparedKeccak::with_expected_digest(vm, input, program.byte_size, digest)
}

pub fn prepare_ecdsa(
    _input_size: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedEcdsa<EreJolt> {
    set_jolt_config(262144, 4096, 32768);
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (digest, (pub_key_x, pub_key_y), signature) = utils::generate_ecdsa_k256_input();
    let input = build_ecdsa_jolt_input(&digest, &pub_key_x, &pub_key_y, &signature);

    PreparedEcdsa::new(vm, input, program.byte_size)
}

pub fn prepare_private_tx(
    depth: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedPrivateTx<EreJolt> {
    set_jolt_config(65536, 4096, 32768);
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (input_bytes, expected_public_values) = utils::generate_private_tx_input(depth);
    let input = build_framed_input(input_bytes);

    PreparedPrivateTx::with_expected_public_values(
        vm,
        input,
        program.byte_size,
        expected_public_values,
    )
}

pub fn prepare_constant_overhead(
    _input_size: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(1);
    prepare_targeted(program, utils::targeted::generate_constant_overhead_input())
}

pub fn prepare_merkle_fake(
    branch_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(branch_count * 32);
    prepare_targeted(
        program,
        utils::targeted::generate_fake_merkle_input(branch_count),
    )
}

pub fn prepare_hash_sha256(
    hash_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(hash_count);
    prepare_targeted(
        program,
        utils::targeted::generate_hash_count_input(ByteHashKind::Sha256, hash_count),
    )
}

pub fn prepare_merkle_sha256(
    branch_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(branch_count * 32);
    prepare_targeted(
        program,
        utils::targeted::generate_real_merkle_input(ByteHashKind::Sha256, branch_count),
    )
}

pub fn prepare_hash_keccak(
    hash_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(hash_count);
    prepare_targeted(
        program,
        utils::targeted::generate_hash_count_input(ByteHashKind::Keccak, hash_count),
    )
}

pub fn prepare_merkle_keccak(
    branch_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(branch_count * 32);
    prepare_targeted(
        program,
        utils::targeted::generate_real_merkle_input(ByteHashKind::Keccak, branch_count),
    )
}

pub fn prepare_hash_blake3(
    hash_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(hash_count);
    prepare_targeted(
        program,
        utils::targeted::generate_hash_count_input(ByteHashKind::Blake3, hash_count),
    )
}

pub fn prepare_merkle_blake3(
    branch_count: usize,
    program: &CompiledProgram<RustRv64imacCustomized>,
) -> PreparedTargeted<EreJolt> {
    set_targeted_jolt_config(branch_count * 32);
    prepare_targeted(
        program,
        utils::targeted::generate_real_merkle_input(ByteHashKind::Blake3, branch_count),
    )
}

fn set_targeted_jolt_config(work_units: usize) {
    let max_trace_length = if work_units > 1024 {
        16_777_216
    } else if work_units > 128 {
        8_388_608
    } else {
        1_048_576
    };
    set_jolt_config(max_trace_length, 65_536, 1_048_576);
}

fn prepare_targeted(
    program: &CompiledProgram<RustRv64imacCustomized>,
    generated: (Vec<u8>, Vec<u8>),
) -> PreparedTargeted<EreJolt> {
    let vm = EreJolt::new(program.program.clone(), ProverResource::Cpu)
        .expect("jolt prover build failed");

    let (input_bytes, expected_public_values) = generated;
    let input = build_framed_input(input_bytes);

    PreparedTargeted::with_expected_digest(vm, input, program.byte_size, expected_public_values)
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

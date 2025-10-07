use bincode::Options;
use ere_jolt::{EreJolt, JOLT_TARGET};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use utils::zkvm::{PreparedSha256, build_input};
use zkvm_interface::{Compiler, ProverResourceType};

pub use utils::zkvm::{
    execution_cycles, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};

type CachedProgram = (Vec<u8>, usize);
type ProgramCache = HashMap<usize, CachedProgram>;

static PROGRAMS: LazyLock<Mutex<ProgramCache>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn prepare_sha256(input_size: usize) -> PreparedSha256<EreJolt> {
    let guest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest")
        .join(format!("sha256_{}", input_size));

    if !guest_path.exists() {
        panic!(
            "Jolt guest for input size {} not found at {:?}",
            input_size, guest_path
        );
    }

    let mut cache = PROGRAMS.lock().unwrap();
    let (serialized, byte_size) = cache.entry(input_size).or_insert_with(|| {
        let program = JOLT_TARGET
            .compile(&guest_path)
            .expect("jolt compile failed");
        let serialized = bincode::options()
            .serialize(&program)
            .expect("serialize failed");
        let byte_size = serialized.len();
        (serialized, byte_size)
    });

    let program = bincode::options()
        .deserialize(serialized)
        .expect("deserialize failed");
    let vm = EreJolt::new(program, ProverResourceType::Cpu).expect("jolt prover build failed");

    let (message_bytes, _digest) = utils::generate_sha256_input(input_size);
    let input = build_input(message_bytes);

    PreparedSha256::new(vm, input, *byte_size)
}

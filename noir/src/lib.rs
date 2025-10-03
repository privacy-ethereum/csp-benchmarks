use noir_rs::{
    barretenberg::{
        prove::prove_ultra_honk,
        srs::setup_srs_from_bytecode,
        verify::{get_ultra_honk_verification_key, verify_ultra_honk},
    },
    witness::deserialize_witness,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const WORKSPACE_ROOT: &str = "circuits";
const CIRCUIT_SUB_PATH: &str = "hash/sha256";

fn compile_workspace() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root = current_dir.join(WORKSPACE_ROOT);
    let output = Command::new("nargo")
        .args([
            "compile",
            "--workspace",
            "--silence-warnings",
            "--skip-brillig-constraints-check",
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run nargo compile");
    if !output.status.success() {
        panic!(
            "Workspace compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    workspace_root
}

fn read_file_as_string(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_bytecode_from_circuit_json(circuit_path: &Path) -> std::io::Result<String> {
    let buf = read_file_as_string(circuit_path)?;
    let v: Value =
        serde_json::from_str(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    v.get("bytecode").map(|val| val.to_string())
}

pub fn prepare_sha256(input_size: usize) -> (PathBuf, PathBuf) {
    let workspace_root = compile_workspace();

    let package_name = "sha256_var_input";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

    let bytecode = read_bytecode_from_circuit_json(circuit_path)
        .expect("Cannot read bytecode from circuit json");

    // Setup the SRS
    setup_srs_from_bytecode(&bytecode, None, false).unwrap();

    let dir_name = "sha256_var_input";
    let circuit_member_dir = workspace_root.join(CIRCUIT_SUB_PATH).join(dir_name);
    fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

    // The circuit's input array size is fixed to 2048 bytes, but the actual hashed message size is input_len = {input_size}
    let (data, _digest) = utils::generate_sha256_input(2048);
    let toml_content = format!(
        "input = [{}]\ninput_len = {input_size}",
        data.iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
    );

    let toml_path = circuit_member_dir.join("Prover.toml");
    fs::write(&toml_path, toml_content).expect("Failed to write Prover.toml");

    (toml_path, circuit_path)
}

pub fn prove(toml_path: &Path, circuit_path: &Path) -> Vec<u8> {
    let bytecode = read_bytecode_from_circuit_json(circuit_path)
        .expect("Cannot read bytecode from circuit json");

    let buf = read_file_as_string(toml_path).unwrap();
    let witness = bincode::serialize(&buf).unwrap();
    let initial_witness =
        deserialize_witness(witness).expect("Failed to deserialize initial witness");

    // Get the verification key
    let vk = get_ultra_honk_verification_key(&bytecode, true).unwrap();

    let proof = prove_ultra_honk(&bytecode, initial_witness, vk, true).unwrap();
    proof
}

/// Verify a proof with the given scheme
pub fn verify(proof: &Vec<u8>, circuit_path: &Path) -> Result<(), &'static str> {
    // Read the bytecode from the circuit json
    let bytecode = read_bytecode_from_circuit_json(circuit_path)
        .expect("Cannot read bytecode from circuit json");

    // Get the verification key
    let vk = get_ultra_honk_verification_key(&bytecode, true).unwrap();

    // Verify the proof
    let verdict = verify_ultra_honk(proof, vk).unwrap();

    assert!(verdict, "Verification failed");

    Ok(())
}

pub fn preprocessing_size(circuit_path: &Path) -> usize {
    std::fs::metadata(circuit_path)
        .map(|m| m.len())
        .unwrap_or(0) as usize
}

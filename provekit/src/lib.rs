use provekit_common::{NoirProof, NoirProofScheme};
use provekit_prover::NoirProofSchemeProver;
use provekit_r1cs_compiler::NoirProofSchemeBuilder;
use provekit_verifier::NoirProofSchemeVerifier;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use utils::generate_ecdsa_input;

const WORKSPACE_ROOT: &str = "circuits";
const SHA256_CIRCUIT_SUB_PATH: &str = "hash/sha256-provekit";
const ECDSA_CIRCUIT_SUB_PATH: &str = "ecdsa";

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

pub fn prepare_sha256(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let workspace_root = compile_workspace();

    let package_name = "sha256_var_input";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

    let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme: {e}"));

    let dir_name = "sha256_var_input";
    let circuit_member_dir = workspace_root.join(SHA256_CIRCUIT_SUB_PATH).join(dir_name);
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

    (proof_scheme, toml_path, circuit_path)
}

pub fn prepare_ecdsa(_: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let workspace_root = compile_workspace();

    let package_name = "p256_bigcurve";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

    let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme: {e}"));

    let dir_name = "p256_bigcurve";
    let circuit_member_dir = workspace_root.join(ECDSA_CIRCUIT_SUB_PATH).join(dir_name);
    fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

    let (digest, (pub_key_x, pub_key_y), signature) = generate_ecdsa_input();
    let toml_content = format!(
        "hashed_message = [{}]\npub_key_x = [{}]\npub_key_y = [{}]\nsignature = [{}]",
        digest
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        pub_key_x
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        pub_key_y
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        signature
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", "),
    );

    let toml_path = circuit_member_dir.join("Prover.toml");
    fs::write(&toml_path, toml_content).expect("Failed to write Prover.toml");

    (proof_scheme, toml_path, circuit_path)
}

pub fn prove(proof_scheme: &NoirProofScheme, toml_path: &Path) -> NoirProof {
    let witness_map = proof_scheme
        .read_witness(toml_path.to_str().unwrap())
        .expect("Failed to read witness");
    proof_scheme
        .prove(&witness_map)
        .expect("Proof generation failed")
}

/// Verify a proof with the given scheme
pub fn verify(proof: &NoirProof, proof_scheme: &NoirProofScheme) -> Result<(), &'static str> {
    proof_scheme.verify(proof).map_err(|_| "Proof is not valid")
}

pub fn preprocessing_size(circuit_path: &Path) -> usize {
    std::fs::metadata(circuit_path)
        .map(|m| m.len())
        .unwrap_or(0) as usize
}

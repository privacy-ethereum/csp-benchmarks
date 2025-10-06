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

pub fn prepare_sha256(input_size: usize) -> (usize, PathBuf, PathBuf) {
    let workspace_root = compile_workspace();

    let package_name = "sha256_var_input";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

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

    let toml_path = circuit_member_dir.join(format!("Prover_{input_size}.toml"));
    fs::write(&toml_path, toml_content).expect("Failed to write Prover.toml");

    (input_size, toml_path, circuit_path)
}

pub fn prove(input_size: usize, toml_path: &Path, circuit_path: &Path) -> (PathBuf, PathBuf) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root = current_dir.join(WORKSPACE_ROOT);

    let package_name = "sha256_var_input";
    let witness_file_name = format!("{package_name}_{input_size}.gz");
    let output = Command::new("nargo")
        .args([
            "execute",
            "--prover-name",
            toml_path.file_name().unwrap().to_str().unwrap(),
            "--package",
            package_name,
            &witness_file_name,
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run nargo execute");
    if !output.status.success() {
        panic!(
            "Witness generation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let witness_path = workspace_root.join("target").join(witness_file_name);
    let output_path = workspace_root.join("target/");
    let output = Command::new("bb")
        .args([
            "prove",
            "-b",
            circuit_path.to_str().unwrap(),
            "-w",
            witness_path.to_str().unwrap(),
            "--write_vk",
            "-o",
            output_path.to_str().unwrap(),
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run bb prove");
    if !output.status.success() {
        panic!(
            "Barretenberg prove failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let proof_path = workspace_root.join("target").join("proof");
    let vk_path = workspace_root.join("target").join("vk");
    (proof_path, vk_path)
}

/// Verify a proof
pub fn verify(proof_path: &Path, vk_path: &Path) -> Result<(), &'static str> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root = current_dir.join(WORKSPACE_ROOT);

    let output = Command::new("bb")
        .args([
            "verify",
            "-p",
            proof_path.to_str().unwrap(),
            "-k",
            vk_path.to_str().unwrap(),
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run bb verify");
    if !output.status.success() {
        panic!(
            "Barretenberg verify failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn preprocessing_size(circuit_path: &Path) -> usize {
    std::fs::metadata(circuit_path)
        .map(|m| m.len())
        .unwrap_or(0) as usize
}

pub fn proof_size(proof_path: &Path) -> usize {
    std::fs::metadata(proof_path).map(|m| m.len()).unwrap_or(0) as usize
}

use provekit_common::{NoirProof, NoirProofScheme};
use provekit_prover::NoirProofSchemeProver;
use provekit_r1cs_compiler::NoirProofSchemeBuilder;
use provekit_verifier::NoirProofSchemeVerifier;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use utils::generate_ecdsa_input;
use utils::harness::{AuditStatus, BenchProperties};

const WORKSPACE_ROOT: &str = "circuits";
const SHA256_CIRCUIT_SUB_PATH: &str = "hash/sha256-provekit";
const POSEIDON_CIRCUIT_SUB_PATH: &str = "hash/poseidon";
const ECDSA_CIRCUIT_SUB_PATH: &str = "ecdsa";

pub const PROVEKIT_PROPS: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Spartan+WHIR"), // https://github.com/worldfnd/provekit
    field_curve: Cow::Borrowed("Bn254"),           // https://github.com/worldfnd/provekit
    iop: Cow::Borrowed("Spartan"),                 // https://github.com/worldfnd/provekit
    pcs: Some(Cow::Borrowed("WHIR")),              // https://github.com/worldfnd/provekit
    arithm: Cow::Borrowed("R1CS"),                 // https://github.com/worldfnd/provekit
    is_zk: true,                                   // https://github.com/worldfnd/provekit/pull/138
    is_zkvm: false,
    security_bits: 128, // https://github.com/worldfnd/provekit/blob/d7deea66c41d56c1d411dd799d0d6066272323e4/provekit/r1cs-compiler/src/whir_r1cs.rs#L43
    is_pq: true,        // hash-based PCS
    is_maintained: true, // https://github.com/worldfnd/provekit
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

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
    // 1) Rewrite circuit input length to match input_size before compiling
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root_pre = current_dir.join(WORKSPACE_ROOT);
    let circuit_source =
        workspace_root_pre.join("hash/sha256-provekit/sha256_var_input/src/main.nr");

    if let Ok(mut content) = fs::read_to_string(&circuit_source) {
        // Replace only the input param length in `fn main(input: [u8; N], ...)`
        if let Some(fn_pos) = content.find("fn main(")
            && let Some(input_pos_rel) = content[fn_pos..].find("input: [u8;")
        {
            let input_pos = fn_pos + input_pos_rel + "input: [u8;".len();
            // Skip whitespace
            let bytes = content.as_bytes();
            let mut start = input_pos;
            while start < bytes.len() && bytes[start].is_ascii_whitespace() {
                start += 1;
            }
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if start != end {
                content.replace_range(start..end, &input_size.to_string());
                fs::write(&circuit_source, content).expect("Failed to update circuit input length");
            }
        }
    }

    // 2) Compile workspace
    let workspace_root = compile_workspace();

    // 3) Load scheme and prepare TOML matching the chosen size
    let package_name = "sha256_var_input";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

    let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme: {e}"));

    let dir_name = "sha256_var_input";
    let circuit_member_dir = workspace_root.join(SHA256_CIRCUIT_SUB_PATH).join(dir_name);
    fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

    // Generate exactly `input_size` bytes of input; circuit expects fixed array with `input_size` elements
    let (data, _digest) = utils::generate_sha256_input(input_size);
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

pub fn prepare_poseidon(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root_pre = current_dir.join(WORKSPACE_ROOT);
    let circuit_source = workspace_root_pre.join("hash/poseidon/src/main.nr");

    if let Ok(mut content) = fs::read_to_string(&circuit_source) {
        if let Some(import_pos) = content.find("poseidon::bn254::hash_") {
            let start = import_pos + "poseidon::bn254::hash_".len();
            let mut end = start;
            while end < content.len() && content.as_bytes()[end].is_ascii_digit() {
                end += 1;
            }
            if start != end {
                content.replace_range(start..end, &input_size.to_string());
            }
        }

        if let Some(hash_pos) = content.find("    hash_") {
            let start = hash_pos + "    hash_".len();
            let mut end = start;
            while end < content.len() && content.as_bytes()[end].is_ascii_digit() {
                end += 1;
            }
            if start != end {
                content.replace_range(start..end, &input_size.to_string());
            }
        }

        if let Some(field_pos) = content.find("[Field;") {
            let start = field_pos + "[Field;".len();
            let bytes = content.as_bytes();
            let mut num_start = start;
            while num_start < bytes.len() && bytes[num_start].is_ascii_whitespace() {
                num_start += 1;
            }
            let mut end = num_start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if num_start != end {
                content.replace_range(num_start..end, &input_size.to_string());
            }
        }

        fs::write(&circuit_source, content).expect("Failed to update circuit");
    }

    let workspace_root = compile_workspace();

    let package_name = "poseidon";
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package_name}.json"));

    let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme: {e}"));

    let circuit_member_dir = workspace_root.join(POSEIDON_CIRCUIT_SUB_PATH);
    fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

    let field_elements = utils::generate_poseidon_input_strings(input_size);
    let toml_content = format!(
        "inputs = [{}]",
        field_elements
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ")
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

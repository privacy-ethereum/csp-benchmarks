use provekit_common::{HashConfig, NoirProof, NoirProofScheme, Prover, Verifier, file::serialize};
use provekit_prover::Prove;
use provekit_r1cs_compiler::NoirCompiler;
use provekit_verifier::Verify;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use utils::generate_ecdsa_input;
use utils::harness::{AuditStatus, BenchProperties};

const WORKSPACE_ROOT: &str = "circuits";

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

fn workspace_root() -> PathBuf {
    std::env::current_dir()
        .expect("Failed to get current directory")
        .join(WORKSPACE_ROOT)
}

fn compile_package(package: &str) -> (NoirProofScheme, PathBuf) {
    let workspace_root = workspace_root();
    let output = Command::new("nargo")
        .args([
            "compile",
            "--package",
            package,
            "--silence-warnings",
            "--skip-brillig-constraints-check",
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run nargo compile");
    if !output.status.success() {
        panic!(
            "Compilation failed for {package}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package}.json"));
    let proof_scheme = NoirCompiler::from_file(&circuit_path, HashConfig::default())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme for {package}: {e}"));
    (proof_scheme, circuit_path)
}

// Map a package name like "csp_sha256_128" to the circuit's directory under
// `circuits/`. Upstream layout uses the same suffix as the directory name.
fn package_dir(package: &str) -> PathBuf {
    let suffix = package.strip_prefix("csp_").unwrap_or(package);
    workspace_root().join(suffix)
}

fn write_prover_toml(package: &str, content: &str) -> PathBuf {
    let dir = package_dir(package);
    fs::create_dir_all(&dir).expect("Failed to create circuit dir");
    let toml_path = dir.join("Prover.toml");
    fs::write(&toml_path, content).expect("Failed to write Prover.toml");
    toml_path
}

fn join_u8(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_field_strings(fields: &[String]) -> String {
    fields
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn prepare_sha256(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let package = format!("csp_sha256_{input_size}");
    let (proof_scheme, circuit_path) = compile_package(&package);

    let (data, _digest) = utils::generate_sha256_input(input_size);
    let toml = format!("input = [{}]\ninput_len = {input_size}", join_u8(&data),);
    let toml_path = write_prover_toml(&package, &toml);
    (proof_scheme, toml_path, circuit_path)
}

pub fn prepare_keccak(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let package = format!("csp_keccak_{input_size}");
    let (proof_scheme, circuit_path) = compile_package(&package);

    let (data, digest) = utils::generate_keccak_input(input_size);
    let toml = format!(
        "msg = [{}]\nmessage_size = {input_size}\nresult = [{}]",
        join_u8(&data),
        join_u8(&digest),
    );
    let toml_path = write_prover_toml(&package, &toml);
    (proof_scheme, toml_path, circuit_path)
}

pub fn prepare_poseidon(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let package = format!("csp_poseidon_{input_size}");
    let (proof_scheme, circuit_path) = compile_package(&package);

    let fields = utils::generate_poseidon_input_strings(input_size);
    let toml = format!("inputs = [{}]", join_field_strings(&fields));
    let toml_path = write_prover_toml(&package, &toml);
    (proof_scheme, toml_path, circuit_path)
}

pub fn prepare_poseidon2(input_size: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let package = format!("csp_poseidon2_{input_size}");
    let (proof_scheme, circuit_path) = compile_package(&package);

    let fields = utils::generate_poseidon_input_strings(input_size);
    let toml = format!("inputs = [{}]", join_field_strings(&fields));
    let toml_path = write_prover_toml(&package, &toml);
    (proof_scheme, toml_path, circuit_path)
}

pub fn prepare_ecdsa(_: usize) -> (NoirProofScheme, PathBuf, PathBuf) {
    let package = "csp_ecdsa_p256";
    let (proof_scheme, circuit_path) = compile_package(package);

    let (digest, (pub_key_x, pub_key_y), signature) = generate_ecdsa_input();
    let toml = format!(
        "hashed_message = [{}]\npub_key_x = [{}]\npub_key_y = [{}]\nsignature = [{}]",
        join_u8(&digest),
        join_u8(&pub_key_x),
        join_u8(&pub_key_y),
        join_u8(&signature),
    );
    let toml_path = write_prover_toml(package, &toml);
    (proof_scheme, toml_path, circuit_path)
}

pub fn prove(proof_scheme: &NoirProofScheme, toml_path: &Path) -> NoirProof {
    let prover = Prover::from_noir_proof_scheme(proof_scheme.clone());
    prover
        .prove_with_toml(toml_path)
        .expect("Proof generation failed")
}

/// Verify a proof with the given scheme
pub fn verify(proof: &NoirProof, proof_scheme: &NoirProofScheme) -> Result<(), &'static str> {
    let mut verifier = Verifier::from_noir_proof_scheme(proof_scheme.clone());
    verifier.verify(proof).map_err(|_| "Proof is not valid")
}

pub fn preprocessing_size(proof_scheme: &NoirProofScheme) -> usize {
    let prover = Prover::from_noir_proof_scheme(proof_scheme.clone());
    serialize(&prover)
        .expect("serialize Prover to .pkp bytes")
        .len()
}

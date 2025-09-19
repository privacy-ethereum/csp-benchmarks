use noir_r1cs::{NoirProof, NoirProofScheme};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const WORKSPACE_ROOT: &str = "circuits";
const CIRCUIT_SUB_PATH: &str = "hash/sha256-provekit";

/// Provekit benchmark harness for SHA256.
pub struct ProvekitSha256Benchmark {
    proof_scheme: NoirProofScheme,
    toml_path: PathBuf,
    preprocessing_size: usize,
}

impl ProvekitSha256Benchmark {
    /// Compiles the circuits and creates a new benchmark harness.
    pub fn new(input_size: usize) -> Self {
        // Compile the workspace
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

        let package_name = format!("sha256_bench_{input_size}");
        let circuit_path = workspace_root
            .join("target")
            .join(format!("{package_name}.json"));

        let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
            .unwrap_or_else(|e| panic!("Failed to load proof scheme for size {input_size}: {e}"));

        let dir_name = format!("sha256-bench-{input_size}");
        let circuit_member_dir = workspace_root.join(CIRCUIT_SUB_PATH).join(dir_name);
        fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

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

        let preprocessing_size = std::fs::metadata(circuit_path)
            .map(|m| m.len())
            .unwrap_or(0) as usize
            + std::fs::metadata(&toml_path).map(|m| m.len()).unwrap_or(0) as usize;

        Self {
            proof_scheme,
            toml_path,
            preprocessing_size,
        }
    }

    /// Runs the proving algorithm.
    pub fn run_prove(&self) -> NoirProof {
        let witness_map = self
            .proof_scheme
            .read_witness(self.toml_path.to_str().unwrap())
            .expect("Failed to read witness");

        self.proof_scheme
            .prove(&witness_map)
            .expect("Proof generation failed")
    }

    /// Prepares inputs for verification.
    pub fn prepare_verify(&self) -> (NoirProof, &NoirProofScheme) {
        let proof = self.run_prove();
        (proof, &self.proof_scheme)
    }

    /// Runs the verification algorithm.
    pub fn run_verify(
        &self,
        proof: &NoirProof,
        proof_scheme: &NoirProofScheme,
    ) -> Result<(), &'static str> {
        proof_scheme.verify(proof).map_err(|_| "Proof is not valid")
    }

    /// Returns the preprocessing size.
    pub fn preprocessing_size(&self) -> usize {
        self.preprocessing_size
    }
}

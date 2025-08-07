use noir_r1cs::{NoirProof, NoirProofScheme};
use rand::RngCore;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const WORKSPACE_ROOT: &str = "circuits";
pub const CIRCUIT_SUB_PATH: &str = "hash/sha256-provekit";

/// Provekit benchmark harness for SHA256.
pub struct ProvekitSha256Benchmark {
    proof_schemes: HashMap<u32, NoirProofScheme>,
    toml_paths: HashMap<u32, PathBuf>,
}

impl ProvekitSha256Benchmark {
    /// Compiles the circuits and creates a new benchmark harness.
    pub fn new(exponents: &[u32]) -> Self {
        let output = Command::new("nargo")
            .args([
                "compile",
                "--workspace",
                "--silence-warnings",
                "--skip-brillig-constraints-check",
            ])
            .current_dir(WORKSPACE_ROOT)
            .output()
            .expect("Failed to run nargo compile");

        if !output.status.success() {
            panic!(
                "Workspace compilation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let mut rng = rand::thread_rng();
        let workspace_path = PathBuf::from(WORKSPACE_ROOT);
        let mut proof_schemes = HashMap::new();
        let mut toml_paths = HashMap::new();

        for &exp in exponents {
            let size = 1usize << exp;
            let package_name = format!("sha256_bench_2e{exp}");
            let circuit_path = workspace_path
                .join("target")
                .join(format!("{package_name}.json"));

            let proof_scheme = NoirProofScheme::from_file(circuit_path.to_str().unwrap())
                .unwrap_or_else(|e| panic!("Failed to load proof scheme for exp {exp}: {e}"));
            proof_schemes.insert(exp, proof_scheme);

            let dir_name = format!("sha256-bench-2e{exp}");
            let circuit_member_dir = workspace_path.join(CIRCUIT_SUB_PATH).join(dir_name);
            fs::create_dir_all(&circuit_member_dir).expect("Failed to create circuit dir");

            let mut data = vec![0u8; size];
            rng.fill_bytes(&mut data);
            let toml_content = format!(
                "input = [{}]\ninput_len = {size}",
                data.iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
            );

            let toml_path = circuit_member_dir.join("Prover.toml");
            fs::write(&toml_path, toml_content).expect("Failed to write Prover.toml");
            toml_paths.insert(exp, toml_path);
        }

        Self {
            proof_schemes,
            toml_paths,
        }
    }

    /// Runs the proving algorithm.
    pub fn run_prove(&self, input_exp: u32) -> NoirProof {
        let proof_scheme = self.proof_schemes.get(&input_exp).unwrap();
        let toml_path = self.toml_paths.get(&input_exp).unwrap();
        let witness_map = proof_scheme
            .read_witness(toml_path.to_str().unwrap())
            .expect("Failed to read witness");

        proof_scheme
            .prove(&witness_map)
            .expect("Proof generation failed")
    }

    /// Prepares inputs for verification.
    pub fn prepare_verify(&self, input_exp: u32) -> (NoirProof, &NoirProofScheme) {
        let proof_scheme = self.proof_schemes.get(&input_exp).unwrap();
        let proof = self.run_prove(input_exp);
        (proof, proof_scheme)
    }

    /// Runs the verification algorithm.
    pub fn run_verify(
        &self,
        proof: &NoirProof,
        proof_scheme: &NoirProofScheme,
    ) -> Result<(), &'static str> {
        proof_scheme.verify(proof).map_err(|_| "Proof is not valid")
    }
}

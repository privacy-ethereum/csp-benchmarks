//! SHA256 benchmark file generation

use crate::hash_input_gen::generate_hash_inputs;
use std::fs;
use std::io::Read;
use std::process::Command;

const INPUT_EXP: [u32; 4] = [8, 10, 12, 14];

// Unify all steps in a single function
// Implement `Metrics` struct

/// Compile all SHA256 circuits
fn compile_all_circuits() -> Result<(), Box<dyn std::error::Error>> {
    for exp in INPUT_EXP {
        let circuit_dir = format!("circuits/hash/sha256-provekit/sha256-bench-2e{}", exp);
        println!("Compiling circuit for 2^{}", exp);

        let output = Command::new("nargo")
            .args([
                "compile",
                "--silence-warnings",
                "--skip-brillig-constraints-check",
            ])
            .current_dir(&circuit_dir)
            .output()?;

        if !output.status.success() {
            return Err(format!("Failed to compile circuit 2^{}", exp).into());
        }
    }

    Ok(())
}

/// Generate Prover.toml file for a circuit
fn generate_prover_toml(
    circuit_dir: &str,
    input_file: &str,
    size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open(input_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let content = format!(
        "input = [\n{}\n]\ninput_len = {}\n",
        buffer
            .iter()
            .map(|b| format!("    {}", b))
            .collect::<Vec<_>>()
            .join(",\n"),
        size
    );

    fs::write(format!("{}/Prover.toml", circuit_dir), content)?;
    Ok(())
}

/// Setup Prover.toml files for all circuits
fn setup_circuits() -> Result<(), Box<dyn std::error::Error>> {
    for exp in INPUT_EXP {
        let size = 1usize << exp;
        let input_file = format!("output/hash-input/input_2e{}.bin", exp);
        let circuit_dir = format!("circuits/hash/sha256-provekit/sha256-bench-2e{}", exp);
        println!("Setting up Prover.toml for 2^{}", exp);

        generate_prover_toml(&circuit_dir, &input_file, size)?;
    }
    Ok(())
}

/// Generate all files needed for SHA256 benchmarks
pub fn bench_sha256() -> Result<(), Box<dyn std::error::Error>> {
    generate_hash_inputs()?;
    compile_all_circuits()?;
    setup_circuits()?;
    println!("Setup complete. Run: cargo bench");

    Ok(())
}

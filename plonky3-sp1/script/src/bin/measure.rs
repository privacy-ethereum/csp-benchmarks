use memory_stats::memory_stats;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn main() {
    // Setup the prover client.
    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();

    let usage_before = memory_stats().unwrap();
    
    // Setup the program for proving.
    let (_, _) = client.setup(SHA_ELF);
    
    let usage_after = memory_stats().unwrap();

    println!(
        "Preprocessing memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    let usage_before = memory_stats().unwrap();

    // Load the proving key and verifying key from the files.
    let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
    let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
    // Generate the proof
    let _ = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    
    let usage_after = memory_stats().unwrap();
    
    println!(
        "Proving memory usage: {} GB resident | {} GB virt",
        (usage_after.physical_mem - usage_before.physical_mem) as f32 / (1024.0 * 1024.0 * 1024.0),
        (usage_after.virtual_mem - usage_before.virtual_mem) as f32 / (1024.0 * 1024.0 * 1024.0)
    );
}

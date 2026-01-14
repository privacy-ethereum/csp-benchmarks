use circom_prover::witness::WitnessFn;
use std::collections::HashMap;
use utils::generate_sha256_input;

pub use crate::{prove, verify};

// SHA256 witness generators
witnesscalc_adapter::witness!(sha256_128);
witnesscalc_adapter::witness!(sha256_256);
witnesscalc_adapter::witness!(sha256_512);
witnesscalc_adapter::witness!(sha256_1024);
witnesscalc_adapter::witness!(sha256_2048);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    let witness_fn = match input_size {
        128 => WitnessFn::WitnessCalc(sha256_128_witness),
        256 => WitnessFn::WitnessCalc(sha256_256_witness),
        512 => WitnessFn::WitnessCalc(sha256_512_witness),
        1024 => WitnessFn::WitnessCalc(sha256_1024_witness),
        2048 => WitnessFn::WitnessCalc(sha256_2048_witness),
        _ => unreachable!("Unsupported sha256 input size: {}", input_size),
    };

    // Prepare inputs
    let (input, digest) = generate_sha256_input(input_size);
    let inputs = HashMap::from([
        (
            "in".to_string(),
            input
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>(),
        ),
        (
            "hash".to_string(),
            digest
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>(),
        ),
    ]);
    let input_str = serde_json::to_string(&inputs).unwrap();

    // Prepare zkey path
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let zkey_path = format!(
        "{}/circuits/sha256/sha256_{input_size}/sha256_{input_size}_0001.zkey",
        current_dir.as_path().to_str().unwrap()
    );

    (witness_fn, input_str, zkey_path)
}

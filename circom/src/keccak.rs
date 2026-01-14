use circom_prover::witness::WitnessFn;
use std::collections::HashMap;
use utils::generate_keccak_input;

pub use crate::{prove, verify};

witnesscalc_adapter::witness!(keccak_128);
witnesscalc_adapter::witness!(keccak_256);
witnesscalc_adapter::witness!(keccak_512);
witnesscalc_adapter::witness!(keccak_1024);
witnesscalc_adapter::witness!(keccak_2048);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    let witness_fn = match input_size {
        128 => WitnessFn::WitnessCalc(keccak_128_witness),
        256 => WitnessFn::WitnessCalc(keccak_256_witness),
        512 => WitnessFn::WitnessCalc(keccak_512_witness),
        1024 => WitnessFn::WitnessCalc(keccak_1024_witness),
        2048 => WitnessFn::WitnessCalc(keccak_2048_witness),
        _ => unreachable!("Unsupported keccak input size: {}", input_size),
    };

    let (input, digest) = generate_keccak_input(input_size);
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

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let zkey_path = format!(
        "{}/circuits/keccak/keccak_{input_size}/keccak_{input_size}_0001.zkey",
        current_dir.as_path().to_str().unwrap()
    );

    (witness_fn, input_str, zkey_path)
}

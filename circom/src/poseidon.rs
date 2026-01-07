use circom_prover::witness::WitnessFn;
use std::collections::HashMap;
use utils::generate_poseidon_input_strings;

pub use crate::{prove, verify};

witnesscalc_adapter::witness!(poseidon_2);
witnesscalc_adapter::witness!(poseidon_4);
witnesscalc_adapter::witness!(poseidon_8);
witnesscalc_adapter::witness!(poseidon_12);
witnesscalc_adapter::witness!(poseidon_16);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    let witness_fn = match input_size {
        2 => WitnessFn::WitnessCalc(poseidon_2_witness),
        4 => WitnessFn::WitnessCalc(poseidon_4_witness),
        8 => WitnessFn::WitnessCalc(poseidon_8_witness),
        12 => WitnessFn::WitnessCalc(poseidon_12_witness),
        16 => WitnessFn::WitnessCalc(poseidon_16_witness),
        _ => unreachable!("Unsupported poseidon input size: {}", input_size),
    };

    let field_inputs = generate_poseidon_input_strings(input_size);
    let inputs = HashMap::from([("inputs".to_string(), field_inputs)]);
    let input_str = serde_json::to_string(&inputs).unwrap();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let zkey_path = format!(
        "{}/circuits/poseidon/poseidon_{input_size}/poseidon_{input_size}_0001.zkey",
        current_dir.as_path().to_str().unwrap()
    );

    (witness_fn, input_str, zkey_path)
}

use circom_prover::{
    CircomProver,
    prover::{CircomProof, ProofLib},
    witness::WitnessFn,
};
use std::collections::HashMap;
use utils::generate_sha256_input;

// Prepare witness generator
witnesscalc_adapter::witness!(sha256_128);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    // prepare witness_fn
    let witness_fn = match input_size {
        128 => WitnessFn::WitnessCalc(sha256_128_witness),
        _ => unreachable!(),
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
    let zkey_path = format!("./circuits/sha256/sha256_{input_size}/sha256_{input_size}_0001.zkey");

    (witness_fn, input_str, zkey_path)
}

pub fn prove(witness_fn: WitnessFn, input_str: String, zkey_path: String) -> CircomProof {
    // Generate proof
    CircomProver::prove(
        ProofLib::Rapidsnark, // The rapidsnark prover
        witness_fn,
        input_str,
        zkey_path,
    )
    .unwrap()
}

pub fn verify(proof: CircomProof, zkey_path: String) {
    // Verify proof
    let valid = CircomProver::verify(ProofLib::Rapidsnark, proof, zkey_path).unwrap();

    assert!(valid);
}

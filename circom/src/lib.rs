pub mod poseidon;

use circom_prover::{
    CircomProver,
    prover::{CircomProof, ProofLib},
    witness::WitnessFn,
};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use utils::generate_sha256_input;
use utils::harness::{AuditStatus, BenchProperties};

pub const CIRCOM_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Groth16"),
    field_curve: Cow::Borrowed("Bn254"),
    iop: Cow::Borrowed("Groth16"),
    pcs: None,
    arithm: Cow::Borrowed("R1CS"),
    is_zk: true,
    security_bits: 128, // Bn254 curve
    is_pq: false,
    is_maintained: true,
    is_audited: AuditStatus::PartiallyAudited, // e.g., https://veridise.com/wp-content/uploads/2023/02/VAR-circom-bigint.pdf
    isa: None,
};

pub fn sum_file_sizes_in_the_dir(file_path: &str) -> std::io::Result<usize> {
    let dir = Path::new(file_path)
        .parent()
        .expect("File should have a parent directory");

    let mut total_size: usize = 0;

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            total_size += metadata.len() as usize;
        }
    }

    Ok(total_size)
}

// Prepare witness generator
witnesscalc_adapter::witness!(sha256_128);
witnesscalc_adapter::witness!(sha256_256);
witnesscalc_adapter::witness!(sha256_512);
witnesscalc_adapter::witness!(sha256_1024);
witnesscalc_adapter::witness!(sha256_2048);

pub fn prepare(input_size: usize) -> (WitnessFn, String, String) {
    // prepare witness_fn
    let witness_fn = match input_size {
        128 => WitnessFn::WitnessCalc(sha256_128_witness),
        256 => WitnessFn::WitnessCalc(sha256_256_witness),
        512 => WitnessFn::WitnessCalc(sha256_512_witness),
        1024 => WitnessFn::WitnessCalc(sha256_1024_witness),
        2048 => WitnessFn::WitnessCalc(sha256_2048_witness),
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
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let zkey_path = format!(
        "{}/circuits/sha256/sha256_{input_size}/sha256_{input_size}_0001.zkey",
        current_dir.as_path().to_str().unwrap()
    );

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

pub fn read_constraint_count(zkey_path: &str) -> usize {
    use ark_bn254::Bn254;
    use circom_prover::prover::ark_circom;
    use std::fs::File;
    use std::io::BufReader;

    let mut buffer = BufReader::new(File::open(zkey_path).expect("Unable to open zkey"));
    let (_, constraint_matrices) =
        ark_circom::read_zkey::<_, Bn254>(&mut buffer).expect("Unable to read zkey");
    constraint_matrices.num_constraints
}

pub fn proof_size(proof: &CircomProof) -> usize {
    serde_json::to_vec(proof)
        .expect("Failed to serialize proof")
        .len()
}

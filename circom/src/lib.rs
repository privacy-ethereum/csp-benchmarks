pub mod keccak;
pub mod poseidon;
pub mod sha256;

use circom_prover::{
    CircomProver,
    prover::{CircomProof, ProofLib},
    witness::WitnessFn,
};
use std::borrow::Cow;
use std::path::Path;
use utils::harness::{AuditStatus, BenchProperties};

pub const CIRCOM_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Groth16"),
    field_curve: Cow::Borrowed("Bn254"),
    iop: Cow::Borrowed("Groth16"),
    pcs: None,
    arithm: Cow::Borrowed("R1CS"),
    is_zk: true,
    is_zkvm: false,
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

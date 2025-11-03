use std::{fs::File, io::BufReader};

use ark_bn254::Bn254;
use circom::prepare;
use circom_prover::prover::{CircomProof, ark_circom};
use utils::harness::{AuditStatus, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Circom,
    None,
    "sha256_mem_circom",
    utils::harness::BenchProperties::new(
        "Groth16",
        "Bn254",
        "Groth16",
        None,
        "R1CS",
        true,
        128, // Bn254 curve
        false,
        true,
        AuditStatus::PartiallyAudited, // e.g., https://veridise.com/wp-content/uploads/2023/02/VAR-circom-bigint.pdf
        None,
    ),
    |input_size| { prepare(input_size) },
    |(_witness_fn, _input_str, zkey_path)| {
        let (_, constraint_matrices) = ark_circom::read_zkey::<_, Bn254>(&mut BufReader::new(
            File::open(zkey_path).expect("Unable to open zkey"),
        ))
        .expect("Unable to read zkey");
        constraint_matrices.num_constraints
    },
    |(witness_fn, input_str, zkey_path)| {
        circom::prove(*witness_fn, input_str.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path), proof| {
        circom::verify(proof.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path)| {
        // NOTE: We assume that the dir which includes "[circuit].zkey" also contains the files
        //       needed for witness generation("[circuit].cpp", "[circuit].dat" files).
        sum_file_sizes_in_the_dir(zkey_path).expect("Unable to compute preprocessing size")
    },
    |proof: &CircomProof| {
        // Serialize the proof to JSON and measure its byte size
        serde_json::to_vec(proof)
            .expect("Failed to serialize proof")
            .len()
    }
);

fn sum_file_sizes_in_the_dir(file_path: &str) -> std::io::Result<usize> {
    // Get the parent directory
    let dir = std::path::Path::new(file_path)
        .parent()
        .expect("File should have a parent directory");

    // Sum file sizes in that directory
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

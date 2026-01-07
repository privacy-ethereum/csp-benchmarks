use circom::poseidon::prepare;
use circom::{proof_size, read_constraint_count, sum_file_sizes_in_the_dir};
use utils::harness::{AuditStatus, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Poseidon,
    ProvingSystem::Circom,
    None,
    "poseidon_mem_circom",
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
        AuditStatus::PartiallyAudited,
        None,
    ),
    |input_size| { prepare(input_size) },
    |(_witness_fn, _input_str, zkey_path)| read_constraint_count(zkey_path),
    |(witness_fn, input_str, zkey_path)| {
        circom::poseidon::prove(*witness_fn, input_str.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path), proof| {
        circom::poseidon::verify(proof.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path)| {
        sum_file_sizes_in_the_dir(zkey_path).expect("Unable to compute preprocessing size")
    },
    |proof| proof_size(proof)
);
